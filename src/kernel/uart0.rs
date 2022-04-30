fn read32(addr: &usize) -> u32
{
    let p = *addr as *const u32;
	unsafe{ *p }
}

fn write32(addr: &usize, b: &u32)
{
    let p = *addr as *mut u32;
	unsafe{ *p = *b };
}

pub fn sys_jtag_init()
{
	let mut val: u32 = 0;

	/* Config GPIOF0, GPIOF1, GPIOF3 and GPIOF5 to JTAG mode */
	let addr: usize = 0x020000f0 + 0x00;
	val = read32(&addr);
	val &= !(0xf << ((0 & 0x7) << 2));
	val |= ((0x4 & 0xf) << ((0 & 0x7) << 2));
	write32(&addr, &val);

	val = read32(&addr);
	val &= !(0xf << ((1 & 0x7) << 2));
	val |= ((0x4 & 0xf) << ((1 & 0x7) << 2));
	write32(&addr, &val);

	val = read32(&addr);
	val &= !(0xf << ((3 & 0x7) << 2));
	val |= ((0x4 & 0xf) << ((3 & 0x7) << 2));
	write32(&addr, &val);

	val = read32(&addr);
	val &= !(0xf << ((5 & 0x7) << 2));
	val |= ((0x4 & 0xf) << ((5 & 0x7) << 2));
	write32(&addr, &val);
}

pub fn sys_uart_init()
{
    let mut val: u32 = 0;

	/* Config GPIOB8 and GPIOB9 to txd0 and rxd0 */
	let mut addr: usize = 0x02000030 + 0x04;
	val = read32(&addr);
	val &= !(0xf << ((8 & 0x7) << 2));
	val |= ((0x6 & 0xf) << ((8 & 0x7) << 2));
	write32(&addr, &val);

	val = read32(&addr);
	val &= !(0xf << ((9 & 0x7) << 2));
	val |= ((0x6 & 0xf) << ((9 & 0x7) << 2));
	write32(&addr, &val);

	/* Open the clock gate for uart0 */
	addr = 0x0200190c;
	val = read32(&addr);
	val |= 1 << 0;
	write32(&addr, &val);

	/* Deassert uart0 reset */
	addr = 0x0200190c;
	val = read32(&addr);
	val |= 1 << 16;
	write32(&addr, &val);

	/* Config uart0 to 115200-8-1-0 */
	addr = 0x02500000;
	write32(&(addr + 0x04), &0x0);
	write32(&(addr + 0x08), &0xf7);
	write32(&(addr + 0x10), &0x0);
	val = read32(&(addr + 0x0c));
	val |= (1 << 7);
	write32(&(addr + 0x0c), &val);
	write32(&(addr + 0x00), &(0xd & 0xff));
	write32(&(addr + 0x04), &((0xd >> 8) & 0xff));
	val = read32(&(addr + 0x0c));
	val &= !(1 << 7);
	write32(&(addr + 0x0c), &val);
	val = read32(&(addr + 0x0c));
	val &= !(0x1f);
	val |= (0x3 << 0) | (0 << 2) | (0x0 << 3);
	write32(&(addr + 0x0c), &val);
}
pub struct Device {
	// 串口基址
	pub addr: usize
}
pub fn sys_uart_putc( dev: &Device, c: &u8 )
{
	let addr = dev.addr;

	while((read32(&(addr + 0x7c)) & (0x1 << 1)) == 0){
    	/* busy wait */
    }
	write32(&(addr + 0x00), &(*c as u32));
}

// int main(void)
// {
// 	sys_clock_init();
// 	sys_jtag_init();
// 	sys_uart_init();
// 	sys_uart_putc('h');
// 	sys_uart_putc('e');
// 	sys_uart_putc('l');
// 	sys_uart_putc('l');
// 	sys_uart_putc('o');

// 	while(1);
// 	return 0;
// }
