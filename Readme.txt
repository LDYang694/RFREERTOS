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
qemu-system-riscv32 -nographic -machine virt -net none   -chardev stdio,id=con,mux=on -serial chardev:con   -mon chardev=con,mode=readline -bios none   -smp 4 -kernel ./target/riscv32imac-unknown-none-elf/debug/r_freertos -s -S
（新终端）
riscv64-unknown-elf-gdb -q ./target/riscv32imac-unknown-none-elf/debug/r_freertos
target remote localhost:1234
break main_new_1
break prvInitialiseNewQueue
break xQueueGenericReset


(gdb) x/10xw 0x82442000  
0x82442000 <_ZN10r_freertos6kernel9allocator9init_heap4HEAP17hbd1b852d605d00b7E+4194088>:       0x00000001      0x00000001      0x00000000      0x00000000
0x82442010 <_ZN10r_freertos6kernel9allocator9init_heap4HEAP17hbd1b852d605d00b7E+4194104>:       0x820420e0      0x820420e0      0x00000000      0x00000000
0x82442020 <_ZN10r_freertos6kernel9allocator9init_heap4HEAP17hbd1b852d605d00b7E+4194120>:       0x00000001      0x00000004
(gdb) x/10xw 0x82442040  
0x82442040 <_ZN10r_freertos6kernel9allocator9init_heap4HEAP17hbd1b852d605d00b7E+4194152>:       0x00000001      0x00000001      0x00000000      0x00000000
0x82442050 <_ZN10r_freertos6kernel9allocator9init_heap4HEAP17hbd1b852d605d00b7E+4194168>:       0x82442020      0x82442020      0x00000000      0x00000000
0x82442060 <_ZN10r_freertos6kernel9allocator9init_heap4HEAP17hbd1b852d605d00b7E+4194184>:       0x00000000      0x00000000

(gdb) x/10xw 0x82442080
0x82442080 <_ZN10r_freertos6kernel9allocator9init_heap4HEAP17hbd1b852d605d00b7E+4194216>:       0x00000002      0x00000001      0x824420a4      0x824420a8
0x82442090 <_ZN10r_freertos6kernel9allocator9init_heap4HEAP17hbd1b852d605d00b7E+4194232>:       0x824420a4      0x824420a4      0x82442040      0x82442000
0x824420a0 <_ZN10r_freertos6kernel9allocator9init_heap4HEAP17hbd1b852d605d00b7E+4194248>:       0x00000000      0x00000001
