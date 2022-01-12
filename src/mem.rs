use crate::error::ConfigError;
use libkvm::kvm_userspace_memory_region;

#[derive(Copy, Clone)]
pub struct MemorySlotConfig {
    pub(crate) id: i32,
    pub(crate) size: usize,
    pub(crate) guest_location: u64,
}

impl Default for MemorySlotConfig {
    fn default() -> Self {
        Self {
            id: 0,
            size: 0,
            guest_location: 0,
        }
    }
}

pub struct MemorySlotConfigBuilder {
    config: MemorySlotConfig,
}

impl MemorySlotConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
        }
    }

    pub fn id(&mut self, slot: i32) -> &mut Self {
        self.config.id = slot;
        self
    }

    pub fn size(&mut self, size: usize) -> &mut Self {
        self.config.size = size;
        self
    }

    pub fn guest_location(&mut self, loc: u64) -> &mut Self {
        self.config.guest_location = loc;
        self
    }

    pub fn build(&self) -> Result<MemorySlotConfig, ConfigError> {
        Ok(self.config)
    }
}

pub struct MemorySlot {
    pub(crate) config: MemorySlotConfig,
    pub(crate) mem: kvm_userspace_memory_region,
}

impl MemorySlot {
    pub fn id(&self) -> i32 {
        self.config.id
    }

    pub fn size(&self) -> usize {
        self.config.size
    }
    
    pub fn guest_location(&self) -> u64 {
        self.config.guest_location
    }
}
