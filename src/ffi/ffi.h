#define BaseType_t int
#define TickType_t int
#define pdTRUE true
#define pdFALSE false

#define taskSCHEDULER_SUSPENDED      ( ( BaseType_t ) 0 )
#define taskSCHEDULER_NOT_STARTED    ( ( BaseType_t ) 1 )
#define taskSCHEDULER_RUNNING        ( ( BaseType_t ) 2 )

void rustAssert(bool);
void rustPrint(char*);
void* rustMalloc(int size_);
void rustYield();