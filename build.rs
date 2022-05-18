extern crate cc;

fn main() {
    cc::Build::new()
        .compiler("")
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
