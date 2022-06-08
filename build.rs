use std::env;
extern crate cc;

fn main() {
    let target = env::var("TARGET").expect("TARGET was not set");
    if (target.contains("riscv32")) {
        {
            cc::Build::new()
                .compiler("riscv64-unknown-elf-gcc")
                .file("src/kernel/portASM.S")
                //.file("src/kernel/temp.c")
                .file("src/kernel/start.S")
                .flag("-march=rv32ima")
                .compile("portASM");
        }
    }
    else if (target.contains("riscv64")) {
        {
            cc::Build::new()
                .compiler("riscv64-unknown-elf-gcc")
                .file("src/kernel/portASM.S")
                .file("src/kernel/start.S")
                // .file("src/kernel/temp.c")
                .flag("-march=rv64ima")
                .flag("-mabi=lp64")
                // .file("src/tests/test.c")
                // .file("src/tests/td_task.c")
                .compile("portASM");
        }
    }
 
}
