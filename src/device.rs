use super::error::DeviceError;

pub struct Device {
    pub(crate) fd: i32,
}

impl Device {
    pub fn new() -> Result<Self, DeviceError> {
        let fd = unsafe { libkvm::libkvm_open() };

        if fd < 0 {
            Err(DeviceError::new("open kvm device failed"))
        } else {
            Ok(Device { fd })
        }
    }

    pub fn get_api_version(&self) -> Result<u32, DeviceError> {
        let return_val = unsafe { libkvm::libkvm_get_api_version(self.fd) };

        if return_val < 0 {
            Err(DeviceError::new("get kvm api version failed"))
        } else {
            Ok(return_val.try_into().unwrap())
        }
    }
}
