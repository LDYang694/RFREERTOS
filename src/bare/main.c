#include <stdio.h>
#include <stdlib.h>
#include <riscv64.h>
#include <printf.h>
#include <uart.h>
#include <common.h>
#include <clk.h>

void _putchar(char character)
{
    sys_uart_putc_c(0, character);
    // send char to console etc.
}
uint64_t *pullMachineTimerCompareRegister=0,*pullNextTime=0,*xISRStackTop=0;
int main_c(void)
{
    // char ch = -1;
    // int cnt = 0;
    sys_clock_init();
    sys_uart0_init();
    table_val_set();
    _putchar('h');
    _putchar('i');
    _putchar('\n');
    _putchar('\r');
    // all_interrupt_enable();
    // _putchar('o');
    // _putchar('k');
    // _putchar('\n');
    // _putchar('\r');
    // clint_timer_init();
    // _putchar('\n');
    // _putchar('\r');
    // while(1)
    // {
    //     _putchar('t');
    //     // ch = sys_uart_getc(0);
    //     // if(ch != 0xff)
    //     // {
    //     //     _putchar(ch);
    //     //     _putchar('\n');
    //     //     _putchar('\r');
    //     //     // printf("%c\n\r", ch);
    //     // }
    //     sdelay(1000000);
    // }
    return 0;
}
