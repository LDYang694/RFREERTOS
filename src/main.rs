#![no_std]
#![no_main]
use core::panic::PanicInfo;
// pub extern "C" fn main() -> ! {
//     rust_main();
//     loop{}
// }

extern "C"{
	fn _putchar(ch: u8);
    fn sdelay(us: u64);
}

#[no_mangle]
pub extern "C" fn main_rust() {
    unsafe{
        _putchar(('r' as u8));
        _putchar(('u' as u8));
        _putchar(('s' as u8));
        _putchar(('t' as u8));
        _putchar(('\n' as u8));
        _putchar(('\r' as u8));
        loop{
            _putchar(('l' as u8));
            _putchar(('o' as u8));
            _putchar(('o' as u8));
            _putchar(('p' as u8));
            _putchar(('\n' as u8));
            _putchar(('\r' as u8));
            sdelay(1000000);
        }

    }
    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
