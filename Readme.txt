# toolchain配置
rustup install nightly
rustup default nightly

# 安装target
rustup target add riscv32imac-unknown-none-elf

# 构建项目
cargo build

# 在qemu上运行
qemu-system-riscv64 -nographic -machine virt -net none   -chardev stdio,id=con,mux=on \
-serial chardev:con   -mon chardev=con,mode=readline -bios none   -smp 4 \
-kernel (可执行文件路径)

# qemu调试：
qemu-system-riscv64 -nographic -machine virt -net none   -chardev stdio,id=con,mux=on -serial chardev:con   -mon chardev=con,mode=readline -bios none   -smp 4 -kernel ./target/riscv64imac-unknown-none-elf/debug/r_freertos -s -S
（新终端）
riscv64-unknown-elf-gdb -q ./target/riscv64imac-unknown-none-elf/debug/r_freertos
target remote localhost:1234
break add_to_heap
c
b prev_power_of_two
