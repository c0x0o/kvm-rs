.code16gcc
.text
.globl _start
.type _start, @function
_start:
	xorw %ax, %ax
	xorw %di, %di
	movw %ax, %es

set_serial_port:
	# ICW1
	movb $0x11, %al
	movw $0x20, %dx
	outb %al, %dx
	# ICW2
	movb $0x20, %al
	movw $0x21, %dx
	outb %al, %dx
	# ICW3
	movb $0x0, %al
	movw $0x21, %dx
	outb %al, %dx
	# ICW4
	movb $0x3, %al
	movw $0x21, %dx
	outb %al, %dx

set_uart_handler:
	movw $0x90, %di
	movw $rx_handler, %es:(%di)
	movw %cs, %es:2(%di)
	sti
	
	# send initial data to start echo
	xorb %al, %al
	movw $0x3f8, %dx
	outb %al, %dx

	# waiting for new interrupt
loop:
	pause
	jmp loop

rx_handler:
	pushaw
	pushfw
	xorb %al, %al
	movw $0x3f8, %dx
	inb %dx, %al
	outb %al, %dx
	popaw
	popfw
	iretw
