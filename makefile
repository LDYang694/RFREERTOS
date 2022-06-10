build:
	cargo build --release
	~/Downloads/XuanTie/Xuantie-900-gcc-elf-newlib-x86_64-V2.2.6/bin/riscv64-unknown-elf-objcopy target/riscv64gc-unknown-none-elf/release/r_freertos --strip-all -O binary RFreeRTOS_Image

doc:
	cargo doc --no-deps
