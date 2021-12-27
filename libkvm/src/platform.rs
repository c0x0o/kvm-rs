/* automatically generated by rust-bindgen 0.59.2 */

pub type __u8 = ::std::os::raw::c_uchar;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __u32 = ::std::os::raw::c_uint;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kvm_regs {
    pub rax: __u64,
    pub rbx: __u64,
    pub rcx: __u64,
    pub rdx: __u64,
    pub rsi: __u64,
    pub rdi: __u64,
    pub rsp: __u64,
    pub rbp: __u64,
    pub r8: __u64,
    pub r9: __u64,
    pub r10: __u64,
    pub r11: __u64,
    pub r12: __u64,
    pub r13: __u64,
    pub r14: __u64,
    pub r15: __u64,
    pub rip: __u64,
    pub rflags: __u64,
}
#[test]
fn bindgen_test_layout_kvm_regs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_regs>(),
        144usize,
        concat!("Size of: ", stringify!(kvm_regs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_regs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_regs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rax as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rax)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rbx as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rbx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rcx as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rcx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rdx as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rdx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rsi as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rsi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rdi as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rdi)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rsp as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rsp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rbp as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rbp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r8 as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r9 as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r9)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r10 as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r10)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r11 as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r11)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r12 as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r12)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r13 as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r13)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r14 as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r14)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).r15 as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(r15)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rip as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_regs>())).rflags as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_regs),
            "::",
            stringify!(rflags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kvm_segment {
    pub base: __u64,
    pub limit: __u32,
    pub selector: __u16,
    pub type_: __u8,
    pub present: __u8,
    pub dpl: __u8,
    pub db: __u8,
    pub s: __u8,
    pub l: __u8,
    pub g: __u8,
    pub avl: __u8,
    pub unusable: __u8,
    pub padding: __u8,
}
#[test]
fn bindgen_test_layout_kvm_segment() {
    assert_eq!(
        ::std::mem::size_of::<kvm_segment>(),
        24usize,
        concat!("Size of: ", stringify!(kvm_segment))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_segment>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_segment))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).limit as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(limit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).selector as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(selector)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).type_ as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).present as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(present)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).dpl as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(dpl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).db as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(db)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).s as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(s)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).l as *const _ as usize },
        19usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(l)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).g as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(g)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).avl as *const _ as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(avl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).unusable as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(unusable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_segment>())).padding as *const _ as usize },
        23usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_segment),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kvm_dtable {
    pub base: __u64,
    pub limit: __u16,
    pub padding: [__u16; 3usize],
}
#[test]
fn bindgen_test_layout_kvm_dtable() {
    assert_eq!(
        ::std::mem::size_of::<kvm_dtable>(),
        16usize,
        concat!("Size of: ", stringify!(kvm_dtable))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_dtable>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_dtable))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dtable>())).base as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dtable),
            "::",
            stringify!(base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dtable>())).limit as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dtable),
            "::",
            stringify!(limit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_dtable>())).padding as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_dtable),
            "::",
            stringify!(padding)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kvm_sregs {
    pub cs: kvm_segment,
    pub ds: kvm_segment,
    pub es: kvm_segment,
    pub fs: kvm_segment,
    pub gs: kvm_segment,
    pub ss: kvm_segment,
    pub tr: kvm_segment,
    pub ldt: kvm_segment,
    pub gdt: kvm_dtable,
    pub idt: kvm_dtable,
    pub cr0: __u64,
    pub cr2: __u64,
    pub cr3: __u64,
    pub cr4: __u64,
    pub cr8: __u64,
    pub efer: __u64,
    pub apic_base: __u64,
    pub interrupt_bitmap: [__u64; 4usize],
}
#[test]
fn bindgen_test_layout_kvm_sregs() {
    assert_eq!(
        ::std::mem::size_of::<kvm_sregs>(),
        312usize,
        concat!("Size of: ", stringify!(kvm_sregs))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_sregs>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_sregs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).ds as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(ds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).es as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(es)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).fs as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(fs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).gs as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(gs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).ss as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(ss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).tr as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(tr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).ldt as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(ldt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).gdt as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(gdt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).idt as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(idt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr0 as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr2 as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr3 as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr4 as *const _ as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).cr8 as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(cr8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).efer as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(efer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).apic_base as *const _ as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(apic_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<kvm_sregs>())).interrupt_bitmap as *const _ as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_sregs),
            "::",
            stringify!(interrupt_bitmap)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kvm_userspace_memory_region {
    pub slot: __u32,
    pub flags: __u32,
    pub guest_phys_addr: __u64,
    pub memory_size: __u64,
    pub userspace_addr: __u64,
}
#[test]
fn bindgen_test_layout_kvm_userspace_memory_region() {
    assert_eq!(
        ::std::mem::size_of::<kvm_userspace_memory_region>(),
        32usize,
        concat!("Size of: ", stringify!(kvm_userspace_memory_region))
    );
    assert_eq!(
        ::std::mem::align_of::<kvm_userspace_memory_region>(),
        8usize,
        concat!("Alignment of ", stringify!(kvm_userspace_memory_region))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_userspace_memory_region>())).slot as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_userspace_memory_region),
            "::",
            stringify!(slot)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_userspace_memory_region>())).flags as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_userspace_memory_region),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_userspace_memory_region>())).guest_phys_addr as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_userspace_memory_region),
            "::",
            stringify!(guest_phys_addr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_userspace_memory_region>())).memory_size as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_userspace_memory_region),
            "::",
            stringify!(memory_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<kvm_userspace_memory_region>())).userspace_addr as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(kvm_userspace_memory_region),
            "::",
            stringify!(userspace_addr)
        )
    );
}
extern "C" {
    pub fn libkvm_open() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_get_api_version(device: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_vm_create(device: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_vm_run(vm: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_vm_insert_mem(
        vm: ::std::os::raw::c_int,
        mem: *mut kvm_userspace_memory_region,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_mem_create(mem: *mut kvm_userspace_memory_region) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_mem_destroy(mem: *mut kvm_userspace_memory_region) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_vcpu_create(vm: ::std::os::raw::c_int, vcpu: u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_vcpu_get_regs(
        vm: ::std::os::raw::c_int,
        regs: *mut kvm_regs,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_vcpu_set_regs(
        vm: ::std::os::raw::c_int,
        regs: *mut kvm_regs,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_vcpu_get_sregs(
        vm: ::std::os::raw::c_int,
        sregs: *mut kvm_sregs,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libkvm_vcpu_set_sregs(
        vm: ::std::os::raw::c_int,
        sregs: *mut kvm_sregs,
    ) -> ::std::os::raw::c_int;
}
