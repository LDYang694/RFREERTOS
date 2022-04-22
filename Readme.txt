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
b main_new_1
c
disassemble $pc
display/x $a0
display/x $a1
x/10xw 0x820000b0

0x000000008000a614 <+0>:*     addi    sp,sp,-80
   0x000000008000a616 <+2>:*     sd      a0,56(sp)
=> 0x000000008000a618 <+4>:*     srli    a1,a0,0x1
   0x000000008000a61c <+8>:*     or      a0,a0,a1
   0x000000008000a61e <+10>:*    srli    a1,a0,0x2
   0x000000008000a622 <+14>:*    or      a0,a0,a1
   0x000000008000a624 <+16>:*    srli    a1,a0,0x4
   0x000000008000a628 <+20>:*    or      a0,a0,a1
   0x000000008000a62a <+22>:*    srli    a1,a0,0x8
   0x000000008000a62e <+26>:*    or      a0,a0,a1
   0x000000008000a630 <+28>:*    srli    a1,a0,0x10
   0x000000008000a634 <+32>:*    or      a0,a0,a1
   0x000000008000a636 <+34>:*    srli    a1,a0,0x20
   0x000000008000a63a <+38>:*    or      a0,a0,a1
   0x000000008000a63c <+40>:*    not     a0,a0  # a0 = 0xfffffffffffffc00 = -1024, a1 = 0
   0x000000008000a640 <+44>:*    srli    a1,a0,0x1
   0x000000008000a644 <+48>:*    auipc   a2,0x1ff6
   0x000000008000a648 <+52>:*    addi    a2,a2,-1428 # 0x820000b0
--Type <RET> for more, q to quit, c to continue without paging--
   0x000000008000a64c <+56>:*    ld      a2,0(a2)
   0x000000008000a64e <+58>:*    and     a1,a1,a2
   0x000000008000a650 <+60>:*    sub     a1,a0,a1
   0x000000008000a654 <+64>:*    auipc   a0,0x1ff6
   0x000000008000a658 <+68>:*    addi    a0,a0,-1436 # 0x820000b8
   0x000000008000a65c <+72>:*    ld      a2,0(a0)
   0x000000008000a65e <+74>:*    and     a0,a1,a2
   0x000000008000a662 <+78>:*    srli    a1,a1,0x2
   0x000000008000a664 <+80>:*    and     a1,a1,a2
   0x000000008000a666 <+82>:*    add     a0,a0,a1
   0x000000008000a668 <+84>:*    srli    a1,a0,0x4
   0x000000008000a66c <+88>:*    add     a0,a0,a1
   0x000000008000a66e <+90>:*    auipc   a1,0x1ff6
   0x000000008000a672 <+94>:*    addi    a1,a1,-1454 # 0x820000c0
   0x000000008000a676 <+98>:*    ld      a1,0(a1)
   0x000000008000a678 <+100>:*   and     a0,a0,a1
   0x000000008000a67a <+102>:*   auipc   a1,0x1ff6
   0x000000008000a67e <+106>:*   addi    a1,a1,-1458 # 0x820000c8
   0x000000008000a682 <+110>:*   ld      a1,0(a1)
--Type <RET> for more, q to quit, c to continue without paging--
   0x000000008000a684 <+112>:*&   mul     a0,a0,a1
   0x000000008000a688 <+116>:*&   srli    a0,a0,0x38  a0=124
   0x000000008000a68a <+118>:*&   sd      a0,48(sp)
   0x000000008000a68c <+120>:*   sd      a0,64(sp)
   0x000000008000a68e <+122>:*   j       0x8000a690 <buddy_system_allocator::prev_power_of_two+124>
   0x000000008000a690 <+124>:*&   ld      a0,48(sp)
   0x000000008000a692 <+126>:*&   mv      a1,a0
   0x000000008000a694 <+128>:*&   sd      a1,40(sp)
   0x000000008000a696 <+130>:*   sd      a0,72(sp)
   0x000000008000a698 <+132>:*   j       0x8000a69a <buddy_system_allocator::prev_power_of_two+134>
   0x000000008000a69a <+134>:*&   li      a0,64
   0x000000008000a69e <+138>:*&   sd      a0,32(sp)
   0x000000008000a6a0 <+140>:*   li      a0,0
   0x000000008000a6a2 <+142>:*   bne     a0,a0,0x8000a6b8 <buddy_system_allocator::prev_power_of_two+164>
   0x000000008000a6a6 <+146>:*   j       0x8000a6a8 <buddy_system_allocator::prev_power_of_two+148>
   0x000000008000a6a8 <+148>:*&   ld      a0,32(sp)
   0x000000008000a6aa <+150>:*&   ld      a1,40(sp)
   0x000000008000a6ac <+152>:*&   sub     a1,a0,a1
   0x000000008000a6b0 <+156>:*   sd      a1,24(sp)
--Type <RET> for more, q to quit, c to continue without paging--
   0x000000008000a6b2 <+158>:*&  bltu    a0,a1,0x8000a6e4 <buddy_system_allocator::prev_power_of_two+208> <failed here, jump to 0x8000a6e4>
   0x000000008000a6b6 <+162>:*   j       0x8000a6d6 <buddy_system_allocator::prev_power_of_two+194>
   0x000000008000a6b8 <+164>:   auipc   a0,0x3
   0x000000008000a6bc <+168>:   addi    a0,a0,-104 # 0x8000d650 <str.0>
   0x000000008000a6c0 <+172>:   auipc   a2,0x3
   0x000000008000a6c4 <+176>:   addi    a2,a2,-136 # 0x8000d638 <.L__unnamed_1>
   0x000000008000a6c8 <+180>:   li      a1,33
   0x000000008000a6cc <+184>:   auipc   ra,0xffff6
   0x000000008000a6d0 <+188>:   jalr    -1462(ra) # 0x80000116 <core::panicking::panic>
   0x000000008000a6d4 <+192>:   unimp
   0x000000008000a6d6 <+194>:*   ld      a0,24(sp)
   0x000000008000a6d8 <+196>:*   addi    a1,a0,-1
   0x000000008000a6dc <+200>:*   sd      a1,16(sp)
   0x000000008000a6de <+202>:*   bltu    a0,a1,0x8000a718 <buddy_system_allocator::prev_power_of_two+260>
   0x000000008000a6e2 <+206>:*   j       0x8000a702 <buddy_system_allocator::prev_power_of_two+238>
   0x000000008000a6e4 <+208>:   auipc   a0,0x3
   0x000000008000a6e8 <+212>:   addi    a0,a0,-100 # 0x8000d680 <str.1>
   0x000000008000a6ec <+216>:   auipc   a2,0x3
   0x000000008000a6f0 <+220>:   addi    a2,a2,-180 # 0x8000d638 <.L__unnamed_1>
--Type <RET> for more, q to quit, c to continue without paging--
   0x000000008000a6f4 <+224>:   li      a1,33
   0x000000008000a6f8 <+228>:   auipc   ra,0xffff6
   0x000000008000a6fc <+232>:   jalr    -1506(ra) # 0x80000116 <core::panicking::panic>
   0x000000008000a700 <+236>:   unimp
   0x000000008000a702 <+238>:*   ld      a2,16(sp)
   0x000000008000a704 <+240>:*   andi    a0,a2,-64
   0x000000008000a708 <+244>:*   li      a1,1
   0x000000008000a70a <+246>:*   sll     a1,a1,a2
   0x000000008000a70e <+250>:*   sd      a1,8(sp)
   0x000000008000a710 <+252>:*   li      a1,0
   0x000000008000a712 <+254>:*   bne     a0,a1,0x8000a73c <buddy_system_allocator::prev_power_of_two+296>
   0x000000008000a716 <+258>:*   j       0x8000a736 <buddy_system_allocator::prev_power_of_two+290>
   0x000000008000a718 <+260>:   auipc   a0,0x3
   0x000000008000a71c <+264>:   addi    a0,a0,-152 # 0x8000d680 <str.1>
   0x000000008000a720 <+268>:   auipc   a2,0x3
   0x000000008000a724 <+272>:   addi    a2,a2,-120 # 0x8000d6a8 <.L__unnamed_2>
   0x000000008000a728 <+276>:   li      a1,33
   0x000000008000a72c <+280>:   auipc   ra,0xffff6
   0x000000008000a730 <+284>:   jalr    -1558(ra) # 0x80000116 <core::panicking::panic>
--Type <RET> for more, q to quit, c to continue without paging--
   0x000000008000a734 <+288>:   unimp
   0x000000008000a736 <+290>:*   ld      a0,8(sp)
   0x000000008000a738 <+292>:*  addi    sp,sp,80
   0x000000008000a73a <+294>:*   ret
   0x000000008000a73c <+296>:   auipc   a0,0x3
   0x000000008000a740 <+300>:   addi    a0,a0,-92 # 0x8000d6e0 <str.2>
   0x000000008000a744 <+304>:   auipc   a2,0x3
   0x000000008000a748 <+308>:   addi    a2,a2,-132 # 0x8000d6c0 <.L__unnamed_3>
   0x000000008000a74c <+312>:   li      a1,35
   0x000000008000a750 <+316>:   auipc   ra,0xffff6
   0x000000008000a754 <+320>:   jalr    -1594(ra) # 0x80000116 <core::panicking::panic>
   0x000000008000a758 <+324>:   unimp







0x000000008000a61c      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 944
2: $a1 = 472
(gdb) 
0x000000008000a61e      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1016
2: $a1 = 472
(gdb) 
0x000000008000a622      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1016
2: $a1 = 254
(gdb) 
0x000000008000a624      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1022
2: $a1 = 254
(gdb) 
0x000000008000a628      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1022
2: $a1 = 63
(gdb) 
0x000000008000a62a      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1023
2: $a1 = 63
(gdb) 
0x000000008000a62e      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1023
2: $a1 = 3
(gdb) 
0x000000008000a630      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1023
2: $a1 = 3
(gdb) 
0x000000008000a634      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1023
2: $a1 = 0
(gdb) 
0x000000008000a636      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1023
2: $a1 = 0
(gdb) 
0x000000008000a63a      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1023
2: $a1 = 0
(gdb) 
0x000000008000a63c      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 1023
2: $a1 = 0
(gdb) 
0x000000008000a640      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = -1024
2: $a1 = 0
(gdb) 
0x000000008000a644      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = -1024
2: $a1 = 9223372036854775296
(gdb) 
0x000000008000a648      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = -1024
2: $a1 = 9223372036854775296
(gdb) 
0x000000008000a64c      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = -1024
2: $a1 = 9223372036854775296
(gdb) 
0x000000008000a64e      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = -1024
2: $a1 = 9223372036854775296
(gdb) 
0x000000008000a650      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = -1024
2: $a1 = 6148914689821704192
(gdb) 
0x000000008000a654      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = -1024
2: $a1 = -6148914689821705216
(gdb) 
0x000000008000a658      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 2181039700
2: $a1 = -6148914689821705216
(gdb) 
0x000000008000a65c      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 2181038264
2: $a1 = -6148914689821705216
(gdb) 
0x000000008000a65e      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 2181038264
2: $a1 = -6148914689821705216
(gdb) 
0x000000008000a662      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 2459565877336757248
2: $a1 = -6148914689821705216
(gdb) 
0x000000008000a664      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 2459565877336757248
2: $a1 = 3074457345971961600
(gdb) 
0x000000008000a666      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 2459565877336757248
2: $a1 = 2459565876275647744
(gdb) 
0x000000008000a668      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 4919131753612404992
2: $a1 = 2459565876275647744
(gdb) 
0x000000008000a66c      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 4919131753612404992
2: $a1 = 307445734600775312
(gdb) 
0x000000008000a66e      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 5226577488213180304
2: $a1 = 307445734600775312
(gdb) 
0x000000008000a672      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 5226577488213180304
2: $a1 = 2181039726
(gdb) 
0x000000008000a676      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 5226577488213180304
2: $a1 = 2181038272
(gdb) 
0x000000008000a678      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 5226577488213180304
2: $a1 = 1085102593177498419
(gdb) 
0x000000008000a67a      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 578721383160427280
2: $a1 = 1085102593177498419
(gdb) 
0x000000008000a67e      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 578721383160427280
2: $a1 = 2181039738
(gdb) 

0x000000008000a682      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 578721383160427280
2: $a1 = 2181038280
(gdb) 
0x000000008000a684      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 578721383160427280
2: $a1 = 72340173073878799
(gdb) 
0x000000008000a688      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 9003053649192938992
2: $a1 = 72340173073878799
(gdb) 
0x000000008000a68a      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 124
2: $a1 = 72340173073878799
(gdb) 
0x000000008000a68c      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 124
2: $a1 = 72340173073878799
(gdb) 
0x000000008000a68e      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 124
2: $a1 = 72340173073878799
(gdb) 
0x000000008000a690      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 124
2: $a1 = 72340173073878799
(gdb) 
0x000000008000a692      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 124
2: $a1 = 72340173073878799
(gdb) 
0x000000008000a694      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 124
2: $a1 = 124
(gdb) 
0x000000008000a696      349         let res: usize = intrinsics::ctlz(num as u64) as u32 as usize;
1: $a0 = 124
2: $a1 = 124
(gdb) 
351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 124
2: $a1 = 124
(gdb) 
0x000000008000a69a      351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 124
2: $a1 = 124
(gdb) 
0x000000008000a69e      351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 64
2: $a1 = 124
(gdb) 
0x000000008000a6a0      351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 64
2: $a1 = 124
(gdb) 
0x000000008000a6a2      351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 0
2: $a1 = 124
(gdb) 
0x000000008000a6a6      351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 0
2: $a1 = 124
(gdb) 
0x000000008000a6a8      351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 0
2: $a1 = 124
(gdb) 
0x000000008000a6aa      351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 64
2: $a1 = 124
(gdb) 
0x000000008000a6ac      351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 64
2: $a1 = 124
(gdb) 
0x000000008000a6b0      351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 64
2: $a1 = -60
(gdb) 
0x000000008000a6b2      351         1 << (8 * (size_of::<usize>()) - res - 1)
1: $a0 = 64
2: $a1 = -60
