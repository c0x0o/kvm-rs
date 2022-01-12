use super::error::KvmError;

pub struct KvmDevice {
    pub(crate) fd: i32,
}

impl KvmDevice {
    pub fn new() -> Result<Self, KvmError> {
        let fd = unsafe { libkvm::libkvm_open() };

        if fd < 0 {
            Err(KvmError::new("open kvm device failed"))
        } else {
            Ok(Self { fd })
        }
    }

    pub fn get_api_version(&self) -> Result<u32, KvmError> {
        let return_val = unsafe { libkvm::libkvm_get_api_version(self.fd) };

        if return_val < 0 {
            Err(KvmError::new("get kvm api version failed"))
        } else {
            Ok(return_val.try_into().unwrap())
        }
    }
}
