#include "projdefs.h"
#include <stdbool.h>

#define BaseType_t int
#define TickType_t int
#define UBaseType_t unsigned int


#define taskSCHEDULER_SUSPENDED      ( ( BaseType_t ) 0 )
#define taskSCHEDULER_NOT_STARTED    ( ( BaseType_t ) 1 )
#define taskSCHEDULER_RUNNING        ( ( BaseType_t ) 2 )

void rustAssert(bool);
void rustPrint(char*);
void* rustMalloc(int size_);
void rustYield();
void rustVSendString(const char*);

#define vSendString rustVSendString

#define portSTACK_TYPE	uint32_t
#define portBASE_TYPE	int32_t
#define portUBASE_TYPE	uint32_t
#define portMAX_DELAY ( TickType_t ) 0xffffffUL