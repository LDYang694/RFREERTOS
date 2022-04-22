use std::mem::size_of;
fn main() {
    let num: usize = 944;
    let res: u32 = num.leading_zeros();
    println!("{} {}", res, size_of::<usize>());
}
