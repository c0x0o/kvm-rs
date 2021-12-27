use libc;

#[derive(Clone, Debug)]
pub struct DeviceError {
    hint: &'static str,
    errno: i32,
}

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let errno = unsafe {
            let mut msg: [u8; 64] = [0; 64];
            libc::strerror_r(self.errno, &mut msg as *mut u8 as *mut i8, 64);
            String::from_utf8(msg.to_vec()).unwrap()
        };
        write!(f, "[errno: {}] {}", errno, self.hint)
    }
}

impl DeviceError {
    pub fn new(hint: &'static str) -> DeviceError {
        DeviceError {
            hint,
            errno: errno::errno().0,
        }
    }

    pub fn with_errno(hint: &'static str, err: i32) -> DeviceError {
        DeviceError { hint, errno: err }
    }
}
