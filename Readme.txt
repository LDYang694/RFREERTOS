# toolchain配置
rustup install nightly
rustup default nightly

# 安装target
rustup target add riscv32imac-unknown-none-elf

# 构建项目
cargo build

# 在qemu上运行
qemu-system-riscv32 -nographic -machine virt -net none   -chardev stdio,id=con,mux=on \
-serial chardev:con   -mon chardev=con,mode=readline -bios none   -smp 4 \
-kernel (可执行文件路径)

# qemu调试：
qemu-system-riscv32 -nographic -machine virt -net none   -chardev stdio,id=con,mux=on -serial chardev:con   -mon chardev=con,mode=readline -bios none   -smp 4 -kernel /home/chenyy/FreeRTOSv202112.00/dev-cyy/target/riscv32imac-unknown-none-elf/debug/dev-cyy -s -S
（新终端）
riscv64-unknown-elf-gdb -q /home/chenyy/FreeRTOSv202112.00/dev-cyy/target/riscv32imac-unknown-none-elf/debug/dev-cyy
target remote localhost:1234
break *0x

<dev_cyy::init_heap::HEAP+3980>

pub fn lock(&self) -> TicketMutexGuard<T>
（含有while self.next_serving.load(Ordering::Acquire) != ticket {）
被inline到<_ZN96_$LT$buddy_system_allocator..LockedHeap$LT$_$GT$$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h757a48b33c6c2a93E>:
