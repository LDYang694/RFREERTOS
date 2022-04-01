build:
	cargo build

run:
	qemu-system-riscv32 -nographic -machine virt -net none   -chardev stdio,id=con,mux=on \
	-serial chardev:con   -mon chardev=con,mode=readline -bios none   -smp 4 \
	-kernel ./target/riscv32imac-unknown-none-elf/debug/r_freertos