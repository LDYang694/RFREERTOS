extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/portASM.S")
        .file("src/temp.c")
        .flag("-march=rv32ima")
        .compile("portASM");
}