#ifndef RUST_PLATFORM_PLATFORM_H_
#define RUST_PLATFORM_PLATFORM_H_

#include <fcntl.h>
#include <linux/kvm.h>
#include <stddef.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <unistd.h>

#include "platform.h"

int libkvm_open() { return open("/dev/kvm", O_RDWR); }

int libkvm_get_api_version(int kvm_fd) {
  return ioctl(kvm_fd, KVM_GET_API_VERSION, NULL);
}

int libkvm_vm_create(int kvm_fd) { return ioctl(kvm_fd, KVM_CREATE_VM, 0); }

int libkvm_vm_insert_mem(int vm_fd, struct kvm_userspace_memory_region *mem) {
  return ioctl(vm_fd, KVM_SET_USER_MEMORY_REGION, mem);
}

int libkvm_mem_create(struct kvm_userspace_memory_region *mem) {
  void *user_space_mm =
      mmap(NULL, mem->memory_size, PROT_EXEC | PROT_READ | PROT_WRITE,
           MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);
  if (user_space_mm == MAP_FAILED) {
    return -1;
  }

  mem->userspace_addr = (uint64_t)user_space_mm;
  return 0;
}

int libkvm_mem_destroy(struct kvm_userspace_memory_region *mem) {
  return munmap((void *)mem->userspace_addr, mem->memory_size);
}

int libkvm_vcpu_create(int vm, int vcpu_id) {
  return ioctl(vm, KVM_CREATE_VCPU, vcpu_id);
}

int libkvm_vcpu_destroy(int vcpu) { return close(vcpu); }

int libkvm_vcpu_get_regs(int vcpu, struct kvm_regs *regs) {
  return ioctl(vcpu, KVM_GET_REGS, regs);
}

int libkvm_vcpu_set_regs(int vcpu, struct kvm_regs *regs) {
  return ioctl(vcpu, KVM_SET_REGS, regs);
}

int libkvm_vcpu_get_sregs(int vcpu, struct kvm_sregs *sregs) {
  return ioctl(vcpu, KVM_GET_SREGS, sregs);
}

int libkvm_vcpu_set_sregs(int vcpu, struct kvm_sregs *sregs) {
  return ioctl(vcpu, KVM_SET_SREGS, sregs);
}

int libkvm_vm_run(int vcpu) { return ioctl(vcpu, KVM_RUN, NULL); }

#endif
