extern crate cc;

fn main() {
    cc::Build::new()
        .compiler("riscv64-unknown-elf-gcc")
        .file("src/kernel/portASM.S")

        // .file("src/kernel/temp.c")
        .flag("-march=rv64ima")
        .flag("-mabi=lp64")

        // .file("src/tests/test.c")
        // .file("src/tests/td_task.c")


        .compile("portASM");
}
