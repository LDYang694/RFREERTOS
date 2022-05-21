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

void test_xQueuePeek_noop_waiting_higher_priority( void )
{
    /* Create a new queue */
    QueueHandle_t xQueue = xQueueCreate( 1, sizeof( uint32_t ) );

    /* Export for callback */
    QueueHandle_t xQueueHandleStatic = xQueue;

    /* Add a value to the queue */
    uint32_t testVal = getNextMonotonicTestValue();

    ( void ) xQueueSend( xQueue, &testVal, 0 );

    /* Insert an item into the event list */
    td_task_setFakeTaskPriority( DEFAULT_PRIORITY + 1 );
    td_task_addFakeTaskWaitingToReceiveFromQueue( xQueue );

    /* peek from the queue */
    uint32_t checkVal = INVALID_UINT32;

    TEST_ASSERT_EQUAL( 1, uxQueueMessagesWaiting( xQueue ) );

    TEST_ASSERT_EQUAL( pdTRUE, xQueuePeek( xQueue, &checkVal, 0 ) );
    TEST_ASSERT_EQUAL( testVal, checkVal );

    TEST_ASSERT_EQUAL( 1, uxQueueMessagesWaiting( xQueue ) );

    /* Veify that the task Yielded */
    //TEST_ASSERT_EQUAL( 1, td_task_getYieldCount() );

    /* Check that vTaskMissedYield was called */
    //TEST_ASSERT_EQUAL( 1, td_task_getCount_vPortYieldWithinAPI() );

    vQueueDelete( xQueue );
}

int queue_test_func()
{
    test_xQueuePeek_noop_waiting_higher_priority();
    rustPrint("success");
    while(true)
    {

    }
}

int test_(){
    int temp=0xff,recv=0;
    char name1[20]="task_func1",*stack1=rustMalloc(0x10000);
    char name2[20]="task_func2",*stack2=rustMalloc(0x10000);
    TaskHandle_t buffer1=get_task_handle();
    TaskHandle_t buffer2=get_task_handle();
    TaskHandle_t handle1=xTaskCreateStatic(task_func1,name1,0x10000,0,stack1,buffer1,3);
    TaskHandle_t handle2=xTaskCreateStatic(task_func2,name2,0x10000,0,stack2,buffer2,3);
    qhandle=xQueueCreate(2,0);
    //qhandle=xSemaphoreCreateBinary();
    
    vTaskStartScheduler();
    
    return recv;
}