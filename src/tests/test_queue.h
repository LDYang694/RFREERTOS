#define MAX_MULTI_LEN             16
#define MAX_QUEUE_ITEMS           500
#define QUEUE_T_SIZE              sizeof( StaticQueue_t )

#define B_SEMPHR_AVAILABLE        1
#define B_SEMPHR_TAKEN            0

#define INVALID_UINT32            0xFFFFFFFF
#define INVALID_PTR               ( ( void * ) INVALID_UINT32 )

#define configASSERT_E            0xAA101

#define queueUNLOCKED             ( ( int8_t ) -1 )
#define queueLOCKED_UNMODIFIED    ( ( int8_t ) 0 )

#define DEFAULT_PRIORITY          5

#define TICKS_TO_WAIT             10
#define NUM_CALLS_TO_INTERCEPT    TICKS_TO_WAIT / 2