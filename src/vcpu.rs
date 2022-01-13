use crate::error::{ConfigError, KvmError};
use errno;
use libc;
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

unsafe impl Send for Vcpu {}
unsafe impl Sync for Vcpu {}

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

    pub fn run(&mut self) -> Result<KvmRun, KvmError> {
        let ret = unsafe { libkvm::libkvm_vm_run(self.fd) };
        if ret < 0 {
            return Err(KvmError::new("unexpected kvm exited"));
        }

        Ok(KvmRun {
            errno: errno::errno().0,
            kvm_run: self.kvm_run,
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

impl VcpuState {
    pub fn get_common_registers_string(&self) -> String {
        format!(
            "RAX={:#x} RBX={:#x} RCX={:#x} RDX={:#x} RSI={:#x} RDI={:#x} \
                RSP={:#x} RBP={:#x} \
                R8={:#x} R9={:#x} R10={:#x} R11={:#x} R12={:#x} R13={:#x} R14={:#x} R15={:#x} \
                RIP={:#x} RFLAGS={:#x}",
            self.regs.rax,
            self.regs.rbx,
            self.regs.rcx,
            self.regs.rdx,
            self.regs.rsi,
            self.regs.rdi,
            self.regs.rsp,
            self.regs.rbp,
            self.regs.r8,
            self.regs.r9,
            self.regs.r10,
            self.regs.r11,
            self.regs.r12,
            self.regs.r13,
            self.regs.r14,
            self.regs.r15,
            self.regs.rip,
            self.regs.rflags
        )
    }
}

pub enum KvmExitReason {
    // non-KVM Exit Reason
    SystemIntrrupt,
    LibUnhandle,

    // KVM Exit Reason
    Unknown(u64),
    Input {
        offset: u64,
        size: u8,
        port: u16,
        count: u32,
    },
    Output {
        size: u8,
        port: u16,
        count: u32,
        data: Vec<u8>,
    },
}

pub struct KvmRun {
    pub(crate) errno: i32,
    pub(crate) kvm_run: *mut libkvm::kvm_run,
}

impl KvmRun {
    pub fn exit_reason(&self) -> KvmExitReason {
        if self.errno == libc::EINTR {
            return KvmExitReason::SystemIntrrupt;
        }

        let run = unsafe { *self.kvm_run };
        match run.exit_reason {
            libkvm::KVM_EXIT_IO => {
                let data_size = unsafe { run.__bindgen_anon_1.io.size as usize };
                let offset = unsafe { run.__bindgen_anon_1.io.data_offset as usize };
                let mut data = Vec::with_capacity(data_size);
                let mut begin = unsafe { (self.kvm_run as *mut u8).add(offset) };
                for _ in 0..data_size {
                    data.push(unsafe { *begin });
                    unsafe {
                        begin = begin.add(1);
                    }
                }

                unsafe {
                    match run.__bindgen_anon_1.io.direction as u32 {
                        libkvm::KVM_EXIT_IO_IN => KvmExitReason::Input {
                            offset: run.__bindgen_anon_1.io.data_offset,
                            size: run.__bindgen_anon_1.io.size,
                            port: run.__bindgen_anon_1.io.port,
                            count: run.__bindgen_anon_1.io.count,
                        },
                        libkvm::KVM_EXIT_IO_OUT => KvmExitReason::Output {
                            size: run.__bindgen_anon_1.io.size,
                            port: run.__bindgen_anon_1.io.port,
                            count: run.__bindgen_anon_1.io.count,
                            data,
                        },
                        _ => KvmExitReason::LibUnhandle,
                    }
                }
            }
            _ => KvmExitReason::LibUnhandle,
        }
    }

    pub fn write_output_data(&mut self, offset: u64, data: &[u8]) {
        let mut begin = unsafe { (self.kvm_run as *mut u8).add(offset as usize) };
        data.iter().for_each(|byte| unsafe {
            *begin = *byte;
            begin = begin.add(1);
        })
    }
}
