#include <asm/kvm.h>
#include <linux/kvm.h>
#include <stdint.h>

int libkvm_open();
int libkvm_dev_get_api_version(int device);
int libkvm_dev_get_kvm_run_mmap_size(int device);
int libkvm_vm_create(int device);
int libkvm_vm_run(int vcpu);
int libkvm_vm_insert_mem(int vm, struct kvm_userspace_memory_region *mem);
int libkvm_vm_create_irqchip(int vm);
int libkvm_mem_create(struct kvm_userspace_memory_region *mem);
int libkvm_mem_destroy(struct kvm_userspace_memory_region *mem);
int libkvm_vcpu_create(int vm, int vcpu_id);
int libkvm_vcpu_destroy(int vcpu);
int libkvm_vcpu_get_regs(int vcpu, struct kvm_regs *regs);
int libkvm_vcpu_set_regs(int vcpu, struct kvm_regs *regs);
int libkvm_vcpu_get_sregs(int vcpu, struct kvm_sregs *sregs);
int libkvm_vcpu_set_sregs(int vcpu, struct kvm_sregs *sregs);
int libkvm_vcpu_kvm_run_create(int vcpu, struct kvm_run **result, int run_size);
int libkvm_vcpu_kvm_run_destroy(struct kvm_run *run, int run_size);
