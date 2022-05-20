all:
	cargo build
	~/Downloads/XuanTie/Xuantie-900-gcc-elf-newlib-x86_64-V2.2.6/bin/riscv64-unknown-elf-objcopy target/riscv64gc-unknown-none-elf/debug/bare_freertos --strip-all -O binary Image
asm:
	~/Downloads/XuanTie/Xuantie-900-gcc-elf-newlib-x86_64-V2.2.6/bin/riscv64-unknown-elf-objdump -S target/riscv64gc-unknown-none-elf/debug/bare_freertos > bare_freertos.asm
