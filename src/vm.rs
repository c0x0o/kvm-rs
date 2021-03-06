use crate::device::KvmDevice;
use crate::error::{ConfigError, KvmError};
use crate::mem::{MemorySlot, MemorySlotConfig};
use crate::vcpu::{Vcpu, VcpuConfig};
use libkvm;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

pub struct VirtualMachineConfig {
    vcpus: HashMap<i32, VcpuConfig>,
    memory_slots: HashMap<i32, MemorySlotConfig>,
}

pub struct VirtualMachineBuilder {
    device: Rc<KvmDevice>,
    config: VirtualMachineConfig,
}

impl VirtualMachineBuilder {
    pub fn new(device: &Rc<KvmDevice>) -> Self {
        Self {
            device: Rc::clone(device),
            config: VirtualMachineConfig {
                vcpus: Default::default(),
                memory_slots: Default::default(),
            },
        }
    }

    pub fn add_vcpu(&mut self, vcpu: &VcpuConfig) -> Result<(), ConfigError> {
        if self.config.vcpus.contains_key(&vcpu.id) {
            return Err(ConfigError::VcpuExists(format!(
                "vcpu at id {} already exists",
                vcpu.id
            )));
        }

        self.config.vcpus.insert(vcpu.id, *vcpu);

        Ok(())
    }

    pub fn add_memory_slot(&mut self, slot: &MemorySlotConfig) -> Result<(), ConfigError> {
        if self.config.memory_slots.contains_key(&slot.id) {
            return Err(ConfigError::MemorySlotExists(format!(
                "memory slot at id {} already exists",
                slot.id
            )));
        }

        self.config.memory_slots.insert(slot.id, *slot);

        Ok(())
    }

    pub fn build(&self) -> Result<VirtualMachine, KvmError> {
        // create vm
        let vm_fd = unsafe {
            let fd = libkvm::libkvm_vm_create(self.device.fd);
            if fd < 0 {
                return Err(KvmError::new("create vm fd failed"));
            } else {
                fd
            }
        };

        // create irq chip
        unsafe {
            if libkvm::libkvm_vm_create_irqchip(vm_fd) < 0 {
                return Err(KvmError::new("create irq chip failed"));
            }
        }

        // create vcpu
        let mut vcpus = HashMap::new();
        for (&id, &config) in self.config.vcpus.iter() {
            let vcpu_fd = unsafe {
                let fd = libkvm::libkvm_vcpu_create(vm_fd, config.id);
                if fd < 0 {
                    return Err(KvmError::new("create vcpu failed"));
                } else {
                    fd
                }
            };

            // mmap kvm run
            let mut kvm_run: *mut libkvm::kvm_run = unsafe { std::mem::zeroed() };
            unsafe {
                if libkvm::libkvm_vcpu_kvm_run_create(
                    vcpu_fd,
                    &mut kvm_run as *mut *mut libkvm::kvm_run,
                    self.device.kvm_run_mmap_size,
                ) < 0
                {
                    return Err(KvmError::new("create kvm_run mmaping failed"));
                }
            }

            vcpus.insert(
                id,
                Arc::new(RwLock::new(Vcpu {
                    kvm_run,
                    kvm_run_mmap_size: self.device.kvm_run_mmap_size,
                    fd: vcpu_fd,
                    config,
                    ..Default::default()
                })),
            );
        }

        // create memory slot
        let mut memory_slots = HashMap::new();
        for (&id, &config) in self.config.memory_slots.iter() {
            let mut region = libkvm::kvm_userspace_memory_region {
                slot: id as u32,
                flags: 0,
                guest_phys_addr: config.guest_location,
                memory_size: config.size as u64,
                userspace_addr: 0,
            };

            unsafe {
                // mapping slot to HVA
                if libkvm::libkvm_mem_create(
                    &mut region as *mut libkvm::kvm_userspace_memory_region,
                ) < 0
                {
                    return Err(KvmError::new("create memory slot failed"));
                }

                // add slot to vm
                if libkvm::libkvm_vm_insert_mem(
                    vm_fd,
                    &mut region as *mut libkvm::kvm_userspace_memory_region,
                ) < 0
                {
                    return Err(KvmError::new("insert memory slot to vm failed"));
                }
            }

            memory_slots.insert(
                id,
                Arc::new(RwLock::new(MemorySlot {
                    config,
                    mem: region,
                })),
            );
        }

        Ok(VirtualMachine {
            fd: vm_fd,
            vcpus,
            memory_slots,
        })
    }
}

pub struct VirtualMachine {
    fd: i32,
    vcpus: HashMap<i32, Arc<RwLock<Vcpu>>>,
    memory_slots: HashMap<i32, Arc<RwLock<MemorySlot>>>,
}

impl VirtualMachine {
    pub fn vcpu(&self, id: i32) -> Option<Arc<RwLock<Vcpu>>> {
        self.vcpus.get(&id).cloned()
    }

    pub fn memory_slot(&self, id: i32) -> Option<Arc<RwLock<MemorySlot>>> {
        self.memory_slots.get(&id).cloned()
    }

    pub fn raise_irq(&self, irq: i32) -> Result<(), KvmError> {
        if unsafe { libkvm::libkvm_vm_set_irq_line_level(self.fd, irq, 0) } < 0 {
            return Err(KvmError::new("set irq to 0 failed when raising interrupt"));
        }

        if unsafe { libkvm::libkvm_vm_set_irq_line_level(self.fd, irq, 1) } < 0 {
            return Err(KvmError::new("set irq to 1 failed when raising interrupt"));
        }

        Ok(())
    }

    pub fn load_to_guest_memory(&self, buffer: &mut [u8], location: u64) -> Result<(), KvmError> {
        let mem_end = location + (buffer.len() as u64);

        // fast path
        if let Some((_, slot_lock)) = self.memory_slots.iter().last() {
            let slot = slot_lock.read().unwrap();
            if slot.guest_location() + (slot.size() as u64) < location + (buffer.len() as u64) {
                return Err(KvmError::with_errno("no available slot", libc::EINVAL));
            }
        } else {
            return Err(KvmError::with_errno("no available slot", libc::EINVAL));
        }

        let mut current_start = 0;
        let mut start_point = location as usize;
        self.memory_slots
            .iter()
            .take_while(|(_, slot_lock)| -> bool {
                let slot = slot_lock.read().unwrap();
                let slot_start = slot.guest_location();
                let slot_end = slot.guest_location() + (slot.size() as u64);

                // start point in the region
                slot_start >= location && slot_start < mem_end
                    // end point in the region
                    || slot_end > location && slot_end <= mem_end
                    // slot contains the  region
                    || slot_start <= location && slot_end >= mem_end
            })
            .for_each(|(_, mem)| unsafe {
                let slot = mem.read().unwrap();
                let size = (buffer.len() - current_start).min(slot.mem.memory_size as usize);

                libc::memcpy(
                    (slot.mem.userspace_addr as *mut libc::c_void).add(start_point),
                    buffer[current_start..].as_mut_ptr() as *mut libc::c_void,
                    size,
                );

                start_point -= start_point;
                current_start += size;
            });
        return Ok(());
    }
}
