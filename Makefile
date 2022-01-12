IMAGE_PATH = ./images

kvm:
	cargo build -p libkvm

build: kvm
	cargo build

forever.bin:
	$(CC) -nostdinc -c $(IMAGE_PATH)/forever/forever.s -o $(IMAGE_PATH)/forever/forever.o
	$(LD) -Ttext=0x00 -nostdlib -static $(IMAGE_PATH)/forever/forever.o -o $(IMAGE_PATH)/forever/forever.elf
	objcopy -O binary $(IMAGE_PATH)/forever/forever.elf $(IMAGE_PATH)/forever/forever.bin

serialport.bin:
	$(CC) -nostdinc -c $(IMAGE_PATH)/serialport/serialport.s -o $(IMAGE_PATH)/serialport/serialport.o
	$(LD) -Ttext=0x00 -nostdlib -static $(IMAGE_PATH)/serialport/serialport.o -o $(IMAGE_PATH)/serialport/serialport.elf
	objcopy -O binary $(IMAGE_PATH)/serialport/serialport.elf $(IMAGE_PATH)/serialport/serialport.bin

images: forever.bin serialport.bin

test: main
	cargo test
