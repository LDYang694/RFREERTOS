#include <uart.h>
#include <clk.h>
#include <gpio.h>

#define UART0_MODE_TX   (6)
#define UART0_MODE_RX   (6)

void sys_uart_putc_c(uint8_t uart_num, char c)
{
#ifndef RISCV64_QEMU
    virtual_addr_t addr = UART_BASE + uart_num * 0x4000;

    while((read32(addr + UART_LSR) & UART_LSR_THRE) == 0);
    write32(addr + UART_THR, c);
#else
    while((*UART_LSR & UART_LSR_THRE) == 0);
    write32(QEMU_UART_BASE + THR, c);
#endif
}

char sys_uart_getc(uint8_t uart_num)
{
#ifndef RISCV64_QEMU
    virtual_addr_t addr = UART_BASE + uart_num * 0x4000;

    if((read32(addr + UART_LSR) & UART_LSR_DR))
    {
        return read32(addr + UART_RBR);
    }
    else
    {
        return -1;
    }
#else
    virtual_addr_t addr = QEMU_UART_BASE;
    if (*UART_LSR & LSR_RX_READY){
        return read32(addr + RHR);
    } 
    else 
    {
        return -1;
    }
#endif
}

void sys_uart0_init(void)
{
    u32_t val;
#ifndef RISCV64_QEMU
    virtual_addr_t addr;
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
#else
    //qemu virt next
    write32(QEMU_UART_BASE + IER, 0x00);
    val = read32(QEMU_UART_BASE + LCR);
    write32(QEMU_UART_BASE + LCR, val | (1 << 7));
    write32(QEMU_UART_BASE + DLL, 0x03);
    write32(QEMU_UART_BASE + DLM, 0x00);

    write32(QEMU_UART_BASE + LCR, (3 << 0));

    /*
     * enable receive interrupts.
     */
    // val = read32(QEMU_UART_BASE + IER);
    // write32(QEMU_UART_BASE + IER, val | (1 << 0));
#endif
}
