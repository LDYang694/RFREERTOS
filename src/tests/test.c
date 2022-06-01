#include "queue/queue_utest_common.h"
#include <setjmp.h>

int shouldAbortOnAssertion = false;   
volatile CEXCEPTION_FRAME_T CExceptionFrames[CEXCEPTION_NUM_ID] = {{ 0 }};

QueueHandle_t qhandle;
uint32_t test_value=0xff,last_value=0xff;
uint32_t getNextMonotonicTestValue(){
    last_value=test_value;
    test_value++;
    return last_value;
}

uint32_t getLastMonotonicTestValue(){
    return last_value;
}

void testtask()
{
    run_queue_send_blocking_utest();
    run_queue_receive_blocking_utest();
    run_queue_reset_utest();
    run_queue_send_nonblocking_utest();
    run_queue_status_utest();
    run_queue_receive_nonblocking_utest();
    run_queue_delete_dynamic_utest();
    run_queue_delete_static_utest();
    run_queue_create_dynamic_utest();
    run_queue_create_static_utest();
    run_binary_semaphore_utest();
    run_counting_semaphore_utest();
    run_semaphore_common_utest();
    run_semaphore_create_utest();
    while(true)
    {

    }
}

int test_(){
    
    xTaskCreate( testtask, "test task", 0x10000, NULL,
					3, NULL );
    vTaskStartScheduler();
    return 0;
}