extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/kernel/portASM.S")
        .file("src/kernel/temp.c")
        .flag("-march=rv64ima")
        .flag("-mabi=lp64")
        .compile("portASM");
}
