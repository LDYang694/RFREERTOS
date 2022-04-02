#include <stdint.h>

uint64_t *pullMachineTimerCompareRegister=0,*pullNextTime=0,*_pxCurrentTCB=0,*xISRStackTop=0;
//todo: pxCurrentTCB

int testfunc(int *x)
{
    return *x;
}