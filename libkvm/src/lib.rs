#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/platform.rs"));

impl Default for kvm_regs {
    fn default() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rsp: 0,
            rbp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rip: 0,
            rflags: 0,
        }
    }
}

impl Default for kvm_segment {
    fn default() -> Self {
        Self {
            base: 0,
            limit: 0,
            selector: 0,
            type_: 0,
            present: 0,
            dpl: 0,
            db: 0,
            s: 0,
            l: 0,
            g: 0,
            avl: 0,
            unusable: 0,
            padding: 0,
        }
    }
}

impl Default for kvm_dtable {
    fn default() -> Self {
        Self {
            base: 0,
            limit: 0,
            padding: Default::default(),
        }
    }
}

impl Default for kvm_sregs {
    fn default() -> Self {
        Self {
            cs: Default::default(),
            ds: Default::default(),
            es: Default::default(),
            fs: Default::default(),
            gs: Default::default(),
            ss: Default::default(),
            tr: Default::default(),
            ldt: Default::default(),
            gdt: Default::default(),
            idt: Default::default(),
            cr0: 0,
            cr2: 0,
            cr3: 0,
            cr4: 0,
            cr8: 0,
            efer: 0,
            apic_base: 0,
            interrupt_bitmap: Default::default(),
        }
    }
}
