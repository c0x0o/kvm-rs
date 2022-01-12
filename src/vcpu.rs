use crate::error::{ConfigError, KvmError};
use libkvm;

#[derive(Clone, Copy)]
pub struct VcpuConfig {
    pub(crate) id: i32,
}

impl Default for VcpuConfig {
    fn default() -> Self {
        VcpuConfig { id: 0 }
    }
}

pub struct VcpuConfigBuilder {
    config: VcpuConfig,
}

impl VcpuConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
        }
    }

    pub fn build(&self) -> Result<VcpuConfig, ConfigError> {
        Ok(self.config.clone())
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.config.id = id;
        self
    }
}

pub struct Vcpu {
    pub(crate) kvm_run: *mut libkvm::kvm_run,
    pub(crate) kvm_run_mmap_size: i32,
    pub(crate) fd: i32,
    pub(crate) config: VcpuConfig,
}

impl Default for Vcpu {
    fn default() -> Self {
        Self {
            kvm_run: unsafe { std::mem::zeroed() },
            kvm_run_mmap_size: 0,
            fd: -1,
            config: Default::default(),
        }
    }
}

impl Drop for Vcpu {
    fn drop(&mut self) {
        unsafe {
            libkvm::libkvm_vcpu_destroy(self.fd);
            libkvm::libkvm_vcpu_kvm_run_destroy(self.kvm_run, self.kvm_run_mmap_size);
        }
    }
}

impl Vcpu {
    pub fn id(&self) -> i32 {
        return self.config.id;
    }

    pub fn run(&self) -> Result<KvmRun, KvmError> {
        let ret = unsafe { libkvm::libkvm_vm_run(self.fd) };
        if ret < 0 {
            return Err(KvmError::new("unexpected kvm exited"));
        }

        Ok(KvmRun {
            kvm_run: self.kvm_run
        })
    }

    pub fn state(&self) -> Result<VcpuState, KvmError> {
        let mut state = VcpuState::default();
        unsafe {
            if libkvm::libkvm_vcpu_get_regs(self.fd, &mut state.regs as *mut libkvm::kvm_regs) < 0 {
                return Err(KvmError::new("get vcpu sregs failed"));
            }
            if libkvm::libkvm_vcpu_get_sregs(self.fd, &mut state.sregs as *mut libkvm::kvm_sregs)
                < 0
            {
                return Err(KvmError::new("get vcpu regs failed"));
            }
        }
        Ok(state)
    }

    pub fn set_state(&self, state: &mut VcpuState) -> Result<(), KvmError> {
        unsafe {
            if libkvm::libkvm_vcpu_set_regs(self.fd, &mut state.regs as *mut libkvm::kvm_regs) < 0 {
                return Err(KvmError::new("get vcpu sregs failed"));
            }
            if libkvm::libkvm_vcpu_set_sregs(self.fd, &mut state.sregs as *mut libkvm::kvm_sregs)
                < 0
            {
                return Err(KvmError::new("get vcpu regs failed"));
            }
        }
        Ok(())
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

pub enum KvmExitReason {
    LibUnhandle,
    Unknown(u64),
    IO {
        // 0 stands for input, 1 stands for output
        direction: u8,
        size: u8,
        port: u16,
        count: u32,
        data: Vec<u8>,
    },
}

pub struct KvmRun {
    pub(crate) kvm_run: *mut libkvm::kvm_run,
}

impl KvmRun {
    pub fn exit_reason(&self) -> KvmExitReason {
        let run = unsafe { *self.kvm_run };
        match run.exit_reason {
            libkvm::KVM_EXIT_IO => {
                let data_size = unsafe { run.__bindgen_anon_1.io.size as usize };
                let offset = unsafe { run.__bindgen_anon_1.io.data_offset as usize };
                let mut data = Vec::with_capacity(data_size);
                let mut begin = unsafe {(self.kvm_run as *mut u8).add(offset)};
                for _ in 0..data_size {
                    data.push(unsafe { *begin });
                    unsafe {
                        begin = begin.add(1);
                    }
                }

                unsafe {
                    KvmExitReason::IO {
                        direction: run.__bindgen_anon_1.io.direction,
                        size: run.__bindgen_anon_1.io.size,
                        port: run.__bindgen_anon_1.io.port,
                        count: run.__bindgen_anon_1.io.count,
                        data,
                    }
                }
            }
            _ => KvmExitReason::LibUnhandle,
        }
    }
}
