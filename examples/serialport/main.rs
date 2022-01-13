use kvm_rs::{
    KvmDevice, KvmExitReason, MemorySlotConfigBuilder, VcpuConfigBuilder, VirtualMachineBuilder,
};
use std::env;
use std::fs;
use std::path::Path;
use std::thread;
use std::time;

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

    let vcpu0 = vm.vcpu(0).unwrap();
    let mut vcpu0_state = vcpu0.read().unwrap().state().unwrap();
    vcpu0_state.sregs.cs.selector = 0x1000;
    vcpu0_state.sregs.cs.base = 0x1000 << 4;
    vcpu0_state.regs.rip = 0x0;
    vcpu0_state.regs.rflags = 0x2;
    vcpu0.read().unwrap().set_state(&mut vcpu0_state).unwrap();

    println!("Loading image...");
    let args: Vec<String> = env::args().collect();
    let mut file = fs::read(Path::new(&args.get(1).unwrap())).unwrap();
    vm.load_to_guest_memory(file.as_mut(), 0x1000 << 4).unwrap();
    
    println!("{:?}", vm.memory_slot(0).unwrap().read().unwrap().inspect_memory_area(0x1000<<4, 128).unwrap());

    println!("vm start running...");
    let handler = thread::spawn(move || {
        let mut last_received: u8 = 0;
        loop {
            let mut run = vcpu0.write().unwrap().run().unwrap();
            match run.exit_reason() {
                KvmExitReason::Output {
                    size,
                    port,
                    count,
                    data,
                } => {
                    println!(
                        "Output Exited: size={}, port={}, count={}, data={:?}",
                        size, port, count, data
                    );
                    println!("{}", vcpu0.read().unwrap().state().unwrap().get_common_registers_string());
                    // rust panics when integer overflow, use wrapping_add
                    // to avoid this
                    // ignore ICW from 8259A PIC main port
                    if port == 0x3f8 {
                        last_received = data[0].wrapping_add(1);
 
                    // we sleep 1 sec to imitate the device processing the data
                    thread::sleep(time::Duration::from_secs(1));
                    
                    // notify the guest that response is ready
                    if let Err(e) = vm.raise_irq(4) {
                        println!("{}", e);
                        break;
                    }
                   }
                }
                KvmExitReason::Input {
                    size,
                    port,
                    count,
                    offset,
                } => {
                    println!(
                        "Input Exited: size={}, port={}, count={}, offset={}",
                        size, port, count, offset
                    );
                    run.write_output_data(offset, &[last_received]);
                }
                KvmExitReason::SystemIntrrupt => {
                    println!("System INTR Exited")
                }
                _ => {
                    println!("Unexpected Exited");
                    break;
                }
            }
        }
    });
    handler.join().expect("could not join thread");
}
