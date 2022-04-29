# toolchain配置
rustup install nightly
rustup default nightly

# 安装target
rustup target add riscv32imac-unknown-none-elf
# "riscv64gc-unknown-none-elf"

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
set arch riscv:rv64
target remote localhost:1234
b _start
c
disassemble $pc
display/x $a0
display/x $a1
display/x $a2
b kernel_init
b add_to_heap
b prvInitialiseNewTask
b main_new_1
c
disassemble $pc
display/x $a0
display/x $a1
x/10xw 0x820000b0
