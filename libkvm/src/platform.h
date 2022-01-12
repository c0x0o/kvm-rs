#include <asm/kvm.h>
#include <linux/kvm.h>
#include <stdint.h>

int libkvm_open();
int libkvm_get_api_version(int device);
int libkvm_vm_create(int device);
int libkvm_vm_run(int vcpu);
int libkvm_vm_insert_mem(int vm, struct kvm_userspace_memory_region *mem);
int libkvm_mem_create(struct kvm_userspace_memory_region *mem);
int libkvm_mem_destroy(struct kvm_userspace_memory_region *mem);
int libkvm_vcpu_create(int vm, int vcpu_id);
int libkvm_vcpu_destroy(int vcpu);
int libkvm_vcpu_get_regs(int vcpu, struct kvm_regs *regs);
int libkvm_vcpu_set_regs(int vcpu, struct kvm_regs *regs);
int libkvm_vcpu_get_sregs(int vcpu, struct kvm_sregs *sregs);
int libkvm_vcpu_set_sregs(int vcpu, struct kvm_sregs *sregs);
