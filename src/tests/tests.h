#include <stdint.h>
#include <stddef.h>
#include <assert.h>
#include <setjmp.h>

#define false 0
#define true 1
#define pdFALSE 0
#define pdTRUE 1

#define TEST_FAIL() rustAssert(0);

#define fakeAssertExpectFail(); ;
#define TEST_ASSERT_EQUAL( a , b ) rustAssert( a==b )

#define CEXCEPTION_T         unsigned int
typedef struct {
  jmp_buf* pFrame;
  CEXCEPTION_T volatile Exception;
} CEXCEPTION_FRAME_T;
#define CEXCEPTION_NUM_ID    (1)


int getNextMonotonicTestValue(){
    return 0xff;
}
 