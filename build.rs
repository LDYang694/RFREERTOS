extern crate cc;

fn main() {
    cc::Build::new()
        .compiler("~/Downloads/XuanTie/Xuantie-900-gcc-elf-newlib-x86_64-V2.2.6/bin/riscv64-unknown-elf-gcc")
        .file("src/kernel/portASM.S")
        .file("src/kernel/temp.c")
        .file("src/bare/clint.c")
        .file("src/bare/clk.c")
        .file("src/bare/common.c")
        .file("src/bare/gpio.c")
        .file("src/bare/interrupt.c")
        .file("src/bare/timer.c")
        .file("src/bare/uart.c")
        .flag("-march=rv64ima")
        .flag("-mabi=lp64")
        .compile("portASM");
}
