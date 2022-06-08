build:
	cargo build

run32:
	cargo build 
	qemu-system-riscv32 -nographic -machine virt -net none   -chardev stdio,id=con,mux=on \
	-serial chardev:con   -mon chardev=con,mode=readline -bios none   -smp 4 \
	-kernel ./target/riscv32imac-unknown-none-elf/debug/r_freertos

run64:
	cargo build 
	qemu-system-riscv64 -nographic -machine virt -net none   -chardev stdio,id=con,mux=on \
	-serial chardev:con   -mon chardev=con,mode=readline -bios none   -smp 4 \
	-kernel ./target/riscv64imac-unknown-none-elf/debug/r_freertos

debug:
	cargo build 
	qemu-system-riscv32 -nographic -machine virt -net none   -chardev stdio,id=con,mux=on \
	-serial chardev:con   -mon chardev=con,mode=readline -bios none   -smp 4 \
	-kernel ./target/riscv32imac-unknown-none-elf/debug/r_freertos -s -S

remote:
	riscv64-unknown-elf-gdb -q ./target/riscv32imac-unknown-none-elf/debug/r_freertos

doc:
	cargo doc --no-deps