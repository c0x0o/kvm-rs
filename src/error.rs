use libc;

#[derive(Clone, Debug)]
pub struct KvmError {
    hint: &'static str,
    errno: i32,
}

impl std::fmt::Display for KvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let errno = unsafe {
            let mut msg: [u8; 64] = [0; 64];
            libc::strerror_r(self.errno, &mut msg as *mut u8 as *mut i8, 64);
            String::from_utf8(msg.to_vec()).unwrap()
        };
        write!(f, "[errno: {}] {}", errno, self.hint)
    }
}

impl KvmError {
    pub fn new(hint: &'static str) -> KvmError {
        KvmError {
            hint,
            errno: errno::errno().0,
        }
    }

    pub fn with_errno(hint: &'static str, err: i32) -> KvmError {
        KvmError { hint, errno: err }
    }
}

#[derive(Clone, Debug)]
pub enum ConfigError {
    VcpuExists(String),
    MemorySlotExists(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::VcpuExists(e) => {
                write!(f, "[ConfigError][VcpuExists] {}", e)
            }
            Self::MemorySlotExists(e) => {
                write!(f, "[ConfigError][MemorySlotExists] {}", e)
            }
        }
    }
}
