build:
	cargo build --release
	~/Downloads/XuanTie/Xuantie-900-gcc-elf-newlib-x86_64-V2.2.6/bin/riscv64-unknown-elf-objcopy target/riscv64gc-unknown-none-elf/release/r_freertos --strip-all -O binary Image

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
