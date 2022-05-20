#include "../ffi/queue.h"
#include "../ffi/tasks.h"
#include "../ffi/semphr.h"
#include "tests.h"
#include "test_queue.h"
#include <setjmp.h>

int shouldAbortOnAssertion = false;   
volatile CEXCEPTION_FRAME_T CExceptionFrames[CEXCEPTION_NUM_ID] = {{ 0 }};

QueueHandle_t qhandle;

void task_func1()
{
    int result;
    rustPrint("func1");
    
    while(true)
    {
        result=xQueueSend(qhandle,0,10);
        //result=xSemaphoreGive(qhandle);
        if(result==1){
            rustPrint("send success!");
        }
        else{
            rustPrint("send fail!");
        }
    }
}

void task_func2(){
    int result;
    rustPrint("func2");
    while(true){

        result=xQueueReceive(qhandle,0,10);
        //result=xSemaphoreTake(qhandle,10);
        if(result==1){
            rustPrint("recv success!");
        }
        else{
            rustPrint("recv fail!");
        }

    }
}

int test_(){
    int temp=0xff,recv=0;
    char name1[20]="task_func1",*stack1=rustMalloc(0x10000);
    char name2[20]="task_func1",*stack2=rustMalloc(0x10000);
    TaskHandle_t buffer1=get_task_handle();
    TaskHandle_t buffer2=get_task_handle();
    TaskHandle_t handle1=xTaskCreateStatic(task_func1,name1,0x10000,0,stack1,buffer1,3);
    TaskHandle_t handle2=xTaskCreateStatic(task_func2,name2,0x10000,0,stack2,buffer2,3);
    qhandle=xQueueCreate(2,0);
    //qhandle=xSemaphoreCreateBinary();
    //vTaskStartScheduler();
    
    return recv;
}