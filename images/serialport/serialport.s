.code16gcc
.text
.globl _start
.type _start, @function
_start:
	xorw %ax, %ax
1:
	mov $0x3f8, %dx
	outb %al, %dx
	inc %ax
	jmp 1b
