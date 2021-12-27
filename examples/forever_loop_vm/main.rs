use kvm_rs::{Device, VirtualMachine, MemorySlot, Vcpu};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("Start initializing vm...");

    let dev = Device::new().unwrap();
    let mut vm = VirtualMachine::new(dev).unwrap();

    println!("Initializing memory slot...");
    let mut mem_slot = MemorySlot::default();
    mem_slot.slot(0).guest_location(0).size(1024*1024*32);
    vm.add_memory_slot(&mem_slot).unwrap();
    
    println!("Initializing vcpu0...");
    let vcpu0 = Vcpu::new(0);
    vm.add_new_vcpu(&vcpu0).unwrap();
    
    let mut vcpu0_state = vm.get_vcpu_state(vcpu0.id()).unwrap();
    vcpu0_state.sregs.cs.selector = 0x1000;
    vcpu0_state.sregs.cs.base = 0x1000 << 4;
    vcpu0_state.regs.rip = 0x0;
    vcpu0_state.regs.rflags = 0x2;
    vm.set_vcpu_state(vcpu0.id(), &mut vcpu0_state).unwrap();

    println!("Loading image...");
    let args: Vec<String> = env::args().collect();
    let mut file = fs::read(Path::new(&args.get(0).unwrap())).unwrap();
    vm.load_image(file.as_mut(), 0x1000 << 4).unwrap();
    
    println!("Booting vm...");
    loop {
        vm.run_vcpu(0).unwrap();
        println!("vcpu0 interrupted");
    }
}
