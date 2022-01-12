use super::error::KvmError;
use std::rc::Rc;

pub struct KvmDevice {
    pub(crate) fd: i32,
    pub(crate) api_version: i32,
    pub(crate) kvm_run_mmap_size: i32,
}

impl KvmDevice {
    pub fn new() -> Result<Rc<Self>, KvmError> {
        let fd = KvmDevice::inner_create()?;
        Ok(Rc::new(Self {
            fd,
            api_version: KvmDevice::inner_get_api_version(fd)?,
            kvm_run_mmap_size: KvmDevice::inner_get_kvm_run_mmap_size(fd)?,
        }))
    }
    
    pub fn api_version(&self) -> i32 {
        self.api_version
    }

    pub fn kvm_run_mmap_size(&self) -> i32 {
        self.kvm_run_mmap_size
    }
    
    fn inner_create() -> Result<i32, KvmError> {
        let fd = unsafe { libkvm::libkvm_open() };
        if fd < 0 {
            Err(KvmError::new("open kvm device failed"))
        } else {
            Ok(fd)
        }
    }

    fn inner_get_api_version(fd: i32) -> Result<i32, KvmError> {
        let return_val = unsafe { libkvm::libkvm_dev_get_api_version(fd) };

        if return_val < 0 {
            Err(KvmError::new("get kvm api version failed"))
        } else {
            Ok(return_val.try_into().unwrap())
        }
    }
    
    fn inner_get_kvm_run_mmap_size(fd: i32) -> Result<i32, KvmError> {
        let return_val = unsafe { libkvm::libkvm_dev_get_kvm_run_mmap_size(fd) };

        if return_val < 0 {
            Err(KvmError::new("get kvm_run size failed"))
        } else {
            Ok(return_val.try_into().unwrap())
        }
    }
}
