#include "../ffi/queue.h"
#include "../ffi/tasks.h"
#include "tests.h"
#include "test_queue.h"
#include <setjmp.h>

int shouldAbortOnAssertion = false;   
volatile CEXCEPTION_FRAME_T CExceptionFrames[CEXCEPTION_NUM_ID] = {{ 0 }};

void task_func()
{
    rustPrint("rustprint");
    while(true)
    {

    }
}

int test_(){
    int temp=0xff,recv=0;
    char name[20]="task_func",*stack=rustMalloc(0x10000);
    TaskHandle_t buffer=get_task_handle();
    TaskHandle_t handle=xTaskCreateStatic(task_func,name,0x10000,0,stack,buffer,3);
    if(buffer==handle)
        rustPrint("true");
    else
        rustPrint("false");
    //vTaskStartScheduler();
    
    return recv;
}