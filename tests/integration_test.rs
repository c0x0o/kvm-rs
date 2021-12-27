use kvm_rs as kvm;

#[test]
fn test_get_kvm_api_version_success() {
    let device = kvm::Device::new().unwrap();
    assert_eq!(device.get_api_version().unwrap(), 12)
}

#[test]
fn test_create_kvm_vm_success() {
    let device = kvm::Device::new().unwrap();
    let _ = kvm::VirtualMachine::new(device);
}

#[test]
fn test_vm_insert_memory_success() {
    let device = kvm::Device::new().unwrap();
    let mut vm = kvm::VirtualMachine::new(device).unwrap();
    let mut mem = kvm::MemorySlot::default();

    mem.size(1024 * 1024 * 16);
    vm.add_memory_slot(&mem).unwrap();
}
