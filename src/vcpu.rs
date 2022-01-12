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
    pub(crate) fd: i32,
    pub(crate) config: VcpuConfig,
}

impl Drop for Vcpu {
    fn drop(&mut self) {
        unsafe {
            libkvm::libkvm_vcpu_destroy(self.fd);
        }
    }
}

impl Vcpu {
    pub fn id(&self) -> i32 {
        return self.config.id;
    }

    pub fn run(&self) -> Result<(), KvmError> {
        let ret = unsafe { libkvm::libkvm_vm_run(self.fd) };

        if ret < 0 {
            return Err(KvmError::new("unexpected kvm exited"));
        }

        Ok(())
    }

    pub fn get_state(&self) -> Result<VcpuState, KvmError> {
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
