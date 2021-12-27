use super::device::Device;
use super::error::DeviceError;
use errno::{errno, set_errno};
use std::collections::BTreeMap;

pub struct VirtualMachine {
    fd: i32,
    memory_slots: BTreeMap<u32, libkvm::kvm_userspace_memory_region>,
    vcpus: BTreeMap<u32, i32>,
}

impl VirtualMachine {
    pub fn new(device: Device) -> Result<Self, DeviceError> {
        let fd = unsafe { libkvm::libkvm_vm_create(device.fd) };

        if fd < 0 {
            Err(DeviceError::new("create vm failed"))
        } else {
            Ok(VirtualMachine {
                fd,
                memory_slots: Default::default(),
                vcpus: Default::default(),
            })
        }
    }

    pub fn add_memory_slot(&mut self, mem: &MemorySlot) -> Result<&mut Self, DeviceError> {
        self.check_memory_conflict(mem)?;

        let mut opaque_mem_slot = unsafe {
            let mut opaque = libkvm::kvm_userspace_memory_region {
                slot: mem.slot,
                memory_size: mem.size,
                guest_phys_addr: mem.guest_location,
                flags: 0,
                userspace_addr: 0,
            };
            if libkvm::libkvm_mem_create(&mut opaque as *mut libkvm::kvm_userspace_memory_region)
                < 0
            {
                return Err(DeviceError::new("create vm memory failed"));
            }
            opaque
        };

        unsafe {
            if libkvm::libkvm_vm_insert_mem(
                self.fd,
                &mut opaque_mem_slot as *mut libkvm::kvm_userspace_memory_region,
            ) < 0
            {
                drop_memory_slot(&mut opaque_mem_slot);
                Err(DeviceError::new("insert vm memory failed"))
            } else {
                self.memory_slots.insert(mem.slot, opaque_mem_slot);
                Ok(self)
            }
        }
    }

    pub fn add_new_vcpu(&mut self, vcpu: &Vcpu) -> Result<(), DeviceError> {
        if self.vcpus.contains_key(&vcpu.id) {
            return Err(DeviceError::with_errno("duplicate vcpu id", libc::EINVAL));
        }

        let vcpu_fd = unsafe {
            let return_value = libkvm::libkvm_vcpu_create(self.fd, vcpu.id());
            if return_value < 0 {
                return Err(DeviceError::new("create vcpu failed"));
            } else {
                return_value
            }
        };

        self.vcpus.insert(vcpu.id, vcpu_fd);

        Ok(())
    }

    pub fn set_vcpu_state(&mut self, vcpu_id: u32, state: &mut VcpuState) -> Result<(), DeviceError> {
        let vcpu = if let Some(vcpu) = self.vcpus.get(&vcpu_id) {
            *vcpu
        } else {
            return Err(DeviceError::with_errno("no such vcpu", libc::EINVAL));
        };

        unsafe {
            if libkvm::libkvm_vcpu_set_regs(vcpu, &mut state.regs as *mut libkvm::kvm_regs) < 0 {
                return Err(DeviceError::new("set vcpu sregs failed"));
            }
            if libkvm::libkvm_vcpu_set_sregs(vcpu, &mut state.sregs as *mut libkvm::kvm_sregs)
                < 0
            {
                return Err(DeviceError::new("set vcpu regs failed"));
            }
        }
        Ok(())
    }

    pub fn get_vcpu_state(&mut self, vcpu_id: u32) -> Result<VcpuState, DeviceError> {
        let vcpu = if let Some(vcpu) = self.vcpus.get(&vcpu_id) {
            *vcpu
        } else {
            return Err(DeviceError::with_errno("no such vcpu", libc::EINVAL));
        };

        let mut state = VcpuState::default();
        unsafe {
            if libkvm::libkvm_vcpu_get_regs(vcpu, &mut state.regs as *mut libkvm::kvm_regs) < 0 {
                return Err(DeviceError::new("get vcpu sregs failed"));
            }
            if libkvm::libkvm_vcpu_get_sregs(vcpu, &mut state.sregs as *mut libkvm::kvm_sregs)
                < 0
            {
                return Err(DeviceError::new("get vcpu regs failed"));
            }
        }
        Ok(state)
    }

    pub fn run_vcpu(&mut self, vcpu_id: u32) -> Result<(), DeviceError> {
        let vcpu = if let Some(vcpu) = self.vcpus.get(&vcpu_id) {
            *vcpu
        } else {
            return Err(DeviceError::with_errno("no such vcpu", libc::EINVAL));
        };

        unsafe {
            if libkvm::libkvm_vm_run(vcpu) < 0 {
                return Err(DeviceError::new("error occured when running vcpu"));
            }
        }

        Ok(())
    }

    pub fn load_image(&mut self, buffer: &mut [u8], location: u64) -> Result<(), DeviceError> {
        let mut begin: Option<u32> = None;
        let mut end: Option<u32> = None;

        for (slot, mem) in self.memory_slots.iter() {
            if mem.guest_phys_addr <= location {
                begin = Some(*slot);
            }
            if mem.guest_phys_addr + mem.memory_size > location + buffer.len() as u64 {
                end = Some(*slot);
            }
        }

        if begin.is_none() {
            return Err(DeviceError::with_errno(
                "no suitable memory slot found",
                libc::EINVAL,
            ));
        }

        if end.is_none() {
            end = begin;
        }

        let mut current_start = 0;
        self.memory_slots
            .iter()
            .take_while(|(&slot, &_)| slot >= begin.unwrap() && slot <= end.unwrap())
            .for_each(|(&_, &mem)| unsafe {
                let size = (buffer.len() - current_start).min(mem.memory_size as usize);

                libc::memcpy(
                    mem.userspace_addr as *mut libc::c_void,
                    buffer[current_start..].as_mut_ptr() as *mut libc::c_void,
                    size,
                );
                
                current_start += size;
            });
        return Ok(());
    }

    fn check_memory_conflict(&self, mem: &MemorySlot) -> Result<(), DeviceError> {
        if self.memory_slots.contains_key(&mem.slot) {
            return Err(DeviceError::with_errno(
                "memory slot already in use",
                libc::EINVAL,
            ));
        }

        Ok(())
    }
}

fn drop_memory_slot(opaque: &mut libkvm::kvm_userspace_memory_region) {
    // "destroy" may change errno,
    // so we need to store it
    let old_errno = errno();
    unsafe {
        libkvm::libkvm_mem_destroy(opaque as *mut libkvm::kvm_userspace_memory_region);
    }
    // restore real errno
    set_errno(old_errno);
}

impl Drop for VirtualMachine {
    fn drop(&mut self) {
        self.memory_slots
            .iter_mut()
            .for_each(|(_, opaque)| drop_memory_slot(opaque))
    }
}

pub struct MemorySlot {
    slot: u32,
    size: u64,
    guest_location: u64,
}

impl Default for MemorySlot {
    fn default() -> Self {
        MemorySlot {
            slot: 0,
            size: 0,
            guest_location: 0,
        }
    }
}

impl MemorySlot {
    pub fn slot(&mut self, i: u32) -> &mut Self {
        self.slot = i;
        self
    }

    pub fn size(&mut self, size: u64) -> &mut Self {
        self.size = size;
        self
    }

    pub fn guest_location(&mut self, loc: u64) -> &mut Self {
        self.guest_location = loc;
        self
    }
}

pub struct Vcpu {
    id: u32,
}

impl Vcpu {
    pub fn new(id: u32) -> Self {
        return Vcpu { id };
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
}

pub struct VcpuState {
    pub regs: libkvm::kvm_regs,
    pub sregs: libkvm::kvm_sregs,
}

impl Default for VcpuState {
    fn default() -> Self {
        Self {
            regs: Default::default(),
            sregs: Default::default(),
        }
    }
}

impl VcpuState {
    pub fn new() -> Self {
        Default::default()
    }
}
