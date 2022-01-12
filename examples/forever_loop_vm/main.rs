use kvm_rs::{KvmDevice, MemorySlotConfigBuilder, VcpuConfigBuilder, VirtualMachineBuilder};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("Start initializing vm...");
    let memory_slot0_config = MemorySlotConfigBuilder::new()
        .id(0)
        .size(8 * 1024 * 1024)
        .guest_location(0)
        .build()
        .unwrap();
    let vcpu0_config = VcpuConfigBuilder::new().id(0).build().unwrap();
    let dev = KvmDevice::new().unwrap();
    let mut vm_builder = VirtualMachineBuilder::new(&dev);

    vm_builder.add_vcpu(&vcpu0_config).unwrap();
    vm_builder.add_memory_slot(&memory_slot0_config).unwrap();

    println!("Building vm...");
    let vm = vm_builder.build().unwrap();

    let vcpu0 = vm.get_vcpu(0).unwrap();
    let mut vcpu0_state = vcpu0.read().unwrap().get_state().unwrap();
    vcpu0_state.sregs.cs.selector = 0x1000;
    vcpu0_state.sregs.cs.base = 0x1000 << 4;
    vcpu0_state.regs.rip = 0x0;
    vcpu0_state.regs.rflags = 0x2;
    vcpu0.read().unwrap().set_state(&mut vcpu0_state).unwrap();
    
    println!("Loading image...");
    let args: Vec<String> = env::args().collect();
    let mut file = fs::read(Path::new(&args.get(0).unwrap())).unwrap();
    vm.load_to_guest_memory(file.as_mut(), 0x1000 << 4).unwrap();

    println!("Booting vm...");
    loop {
        vcpu0.read().unwrap().run().unwrap();
        println!("vcpu0 interrupted");
    }
}
