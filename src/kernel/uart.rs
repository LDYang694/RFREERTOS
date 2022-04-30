//D1 debug uart use GPIOB8(TX0) and GPIOB9(RX0)
pub const UART_BASE: usize = 0X02500000;
pub const UART1_BASE: usize = 0X02500400;
pub const UART2_BASE: usize = 0X02500800;
pub const UART3_BASE: usize = 0X02500C00;
pub const UART4_BASE: usize = 0X02501000;
pub const UART5_BASE: usize = 0X02501400;


pub const UART_RBR:u16 = 0x0000;
pub const UART_THR:u16 = 0x0000;
pub const UART_DLL:u16 = 0x0000;
pub const UART_DLH:u16 = 0x0004;
pub const UART_IER:u16 = 0x0004;
pub const UART_IIR:u16 = 0x0008;
pub const UART_FCR:u16 = 0x0008;
pub const UART_LCR:u16 = 0x000C;
pub const UART_MCR:u16 = 0x0010;
pub const UART_LSR:u16 = 0x0014;
pub const UART_MSR:u16 = 0x0018;
pub const UART_SCH:u16 = 0x001C;
pub const UART_USR:u16 = 0x007C;
pub const UART_TFL:u16 = 0x0080;
pub const UART_RFL:u16 = 0x0084;
pub const UART_HSK:u16 = 0x0088;
pub const UART_DMA_REQ_EN:u16 = 0x008C;
pub const UART_HALT:u16 = 0x00A4;
pub const UART_DBG_DLL:u16 = 0x00B0;
pub const UART_DBG_DLH:u16 = 0x00B4;
pub const UART_A_FCC:u16 = 0x00F0;
pub const UART_A_RXDMA_CTRL:u16 = 0x0100;
pub const UART_A_RXDMA_STR:u16 = 0x0104;
pub const UART_A_RXDMA_STA:u16 = 0x0108;
pub const UART_A_RXDMA_LMT:u16 = 0x010C;
pub const UART_A_RXDMA_SADDRL:u16 = 0x0110;
pub const UART_A_RXDMA_SADDRH:u16 = 0x0114;
pub const UART_A_RXDMA_BL:u16 = 0x0118;
pub const UART_A_RXDMA_IE:u16 = 0x0120;
pub const UART_A_RXDMA_IS:u16 = 0x0124;
pub const UART_A_RXDMA_WADDRL:u16 = 0x0128;
pub const UART_A_RXDMA_WADDRH:u16 = 0x012C;
pub const UART_A_RXDMA_RADDRL:u16 = 0x0130;
pub const UART_A_RXDMA_RADDRH:u16 = 0x0134;
pub const UART_A_RXDMA_DCNT:u16 = 0x0138;

pub const UART_LSR_FIFOE:u16 = 0x80;    /* Fifo error */
pub const UART_LSR_TEMT:u16 = 0x40;    /* Transmitter empty */
pub const UART_LSR_THRE:u16 = 0x20;    /* Transmit-hold-register empty */
pub const UART_LSR_BI:u16 = 0x10;    /* Break interrupt indicator */
pub const UART_LSR_FE:u16 = 0x08;    /* Frame error indicator */
pub const UART_LSR_PE:u16 = 0x04;    /* Parity error indicator */
pub const UART_LSR_OE:u16 = 0x02;    /* Overrun error indicator */
pub const UART_LSR_DR:u16 = 0x01;    /* Receiver data ready */
pub const UART_LSR_BRK_ERROR_BITS:u16 = 0x1E;    /* BI, FE, PE, OE bits */

pub const UART0_MODE_TX:u16 = 6;
pub const UART0_MODE_RX:u16 = 6;

pub fn sys_uart_putc(uart_num: u8, c: u8)
{
    let addr: usize = UART_BASE + uart_num * 0x4000;

    while((read32(addr + UART_LSR) & UART_LSR_THRE) == 0);
    write32(addr + UART_THR, c);
}

char sys_uart_getc(uint8_t uart_num)
{
    virtual_addr_t addr = UART_BASE + uart_num * 0x4000;
    if((read32(addr + UART_LSR) & UART_LSR_DR))
    {
        return read32(addr + UART_RBR);
    }
    else
    {
        return -1;
    }
}

void clk_enable_module_uart(virtual_addr_t addr, uint8_t uart_num)
{
    uint32_t val;
    /* Open the clock gate for uart */
    val = read32(addr);
    val |= 1 << (0 + uart_num);
    write32(addr, val);

    /* Deassert uart reset */
    val = read32(addr);
    val |= 1 << (16 + uart_num);
    write32(addr, val);
}

void d1_set_gpio_mode(uint32_t gpio_port, uint32_t gpio_pin, uint16_t mode)
{
    uint32_t pin_level = 0;
    uint32_t gpio_base_addr = 0;
    uint32_t val = 0;
    pin_level = gpio_pin / 8;
    gpio_base_addr = gpio_port + pin_level * 0x04;
    val = read32(gpio_base_addr);

    val &= ~(0xf << ((gpio_pin & 0x7) << 2));
    val |= ((mode & 0xf) << ((gpio_pin & 0x7) << 2));

    write32(gpio_base_addr, val);
}

void sys_uart0_init(void)
{
    virtual_addr_t addr;
    u32_t val;

    d1_set_gpio_mode(GPIO_PORT_B, GPIO_PIN_8, UART0_MODE_TX);
    d1_set_gpio_mode(GPIO_PORT_B, GPIO_PIN_9, UART0_MODE_RX);

    clk_enable_module_uart(D1_CCU_BASE + CCU_UART_BGR_REG, 0);

    /* Config uart0 to 115200-8-1-0 */
    addr = UART_BASE + 0 * 0x4000;
    write32(addr + UART_DLH, 0x0);      //disable all interrupt
    write32(addr + UART_FCR, 0xf7);     //reset fifo
    write32(addr + UART_MCR, 0x0);      //uart mode
    //set 115200
    val = read32(addr + UART_LCR);
    val |= (1 << 7);                    //select Divisor Latch LS Register
    write32(addr + UART_LCR, val);
    write32(addr + UART_DLL, 0xd & 0xff);   // 0x0d=13 240000000/(13*16) = 115200 Divisor Latch Lows
    write32(addr + UART_DLH, (0xd >> 8) & 0xff); //Divisor Latch High
    val = read32(addr + UART_LCR);
    val &= ~(1 << 7);
    write32(addr + UART_LCR, val);

    val = read32(addr + UART_LCR);
    val &= ~0x1f;
    val |= (0x3 << 0) | (0 << 2) | (0x0 << 3); //8 bit, 1 stop bit,parity disabled
    write32(addr + UART_LCR, val);
}
