extern crate cc;

fn main() {
    cc::Build::new()
        .compiler("riscv64-unknown-elf-gcc")
        .file("src/kernel/portASM.S")
        .file("src/tests/test.c")
        .file("src/tests/td_task.c")
        .flag("-march=rv32ima")
        .compile("portASM");
}
