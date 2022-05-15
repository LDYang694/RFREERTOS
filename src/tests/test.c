#include "../ffi/queue.h"
#include "tests.h"
#include "test_queue.h"
#include <setjmp.h>

int shouldAbortOnAssertion = false;   
volatile CEXCEPTION_FRAME_T CExceptionFrames[CEXCEPTION_NUM_ID] = {{ 0 }};

void test_xQueuePeek_success()
{
    /* Create a new queue */
    QueueHandle_t xQueue = xQueueCreate( 1, 4 );

    uint32_t testVal = getNextMonotonicTestValue();

    ( void ) xQueueSend( xQueue, &testVal, 0 );

    uint32_t checkVal = INVALID_UINT32;

    TEST_ASSERT_EQUAL( 1, uxQueueMessagesWaiting( xQueue ) );

    TEST_ASSERT_EQUAL( pdTRUE, xQueueReceive( xQueue, &checkVal, 0 ) );
    TEST_ASSERT_EQUAL( testVal, checkVal );

    TEST_ASSERT_EQUAL( 0, uxQueueMessagesWaiting( xQueue ) );

    //vQueueDelete( xQueue );
}

int test_(){
    int temp=0xff,recv=0;
    /*QueueHandle_t xQueue=xQueueCreate(1,4);
    
    xQueueSend(xQueue,&temp,0);
    xQueuePeek(xQueue,&recv,0);*/
    test_xQueuePeek_success();
    return recv;
}