extern crate cc;

fn main() {
    cc::Build::new()
    .compiler("riscv64-unknown-elf-gcc")
        .file("src/kernel/portASM.S")
        .file("src/kernel/temp.c")
        .flag("-march=rv32ima")
        .compile("portASM");
}