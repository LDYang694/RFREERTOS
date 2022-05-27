extern crate cc;

fn main() {
    cc::Build::new()
        .compiler("riscv64-unknown-elf-gcc")
        .file("src/kernel/portASM.S")
        .file("src/kernel/config_resolve.c")
        .file("src/ffi/main_blinky.c")
        .file("src/ffi/riscv-virt.c")
        //.file("src/tests/td_task.c")
        .flag("-march=rv32ima")
        .compile("portASM");
}
