use kvm_rs as kvm;

#[test]
fn test_get_kvm_api_version_success() {
    let device = kvm::KvmDevice::new().unwrap();
    assert_eq!(device.api_version(), 12)
}

#[test]
fn test_create_kvm_vm_success() {
    let device = kvm::KvmDevice::new().unwrap();
    let _ = kvm::VirtualMachineBuilder::new(&device).build();
}
