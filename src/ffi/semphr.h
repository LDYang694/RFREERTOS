#include "ffi.h"
#include "queue.h"

#define SemaphoreHandle_t void*
#define xSemaphoreCreateBinary xSemaphoreCreateBinaryToC
#define xSemaphoreCreateCounting xSemaphoreCreateCountingToC
#define vSemaphoreDelete vSemaphoreDeleteToC
#define xSemaphoreGive xSemaphoreGiveToC
#define xSemaphoreTake xSemaphoreTakeToC
#define xQueueCreateMutex xQueueCreateMutexToC
#define prvInitialiseMutex prvInitialiseMutexToC

QueueHandle_t xSemaphoreCreateBinaryToC();
#define vSemaphoreCreateBinary( xSemaphore )  \
        {                                                       \
            ( xSemaphore ) = xSemaphoreCreateBinaryToC();       \
            if( ( xSemaphore ) != NULL )                        \
            {                                                   \
                ( void ) xSemaphoreGive( ( xSemaphore ) );      \
            }                                                   \
        }
#define uxSemaphoreGetCount( xSemaphore )     uxQueueMessagesWaiting( ( QueueHandle_t ) ( xSemaphore ) )
#define xSemaphoreTakeFromISR( xSemaphore, pxHigherPriorityTaskWoken )    \
        xQueueReceiveFromISR( ( QueueHandle_t ) ( xSemaphore ), NULL, ( pxHigherPriorityTaskWoken ) )
#define xSemaphoreGiveFromISR( xSemaphore, pxHigherPriorityTaskWoken )    \
        xQueueSendFromISR( ( QueueHandle_t ) ( xSemaphore ), NULL, ( pxHigherPriorityTaskWoken ) )
//note: used xQueueSendFromISR instead of xQueueGiveFromISR
QueueHandle_t xSemaphoreCreateCountingToC(unsigned int uxMaxCount,unsigned int uxInitialCount);
void vSemaphoreDeleteToC(QueueHandle_t xQueue);
int xSemaphoreGiveToC(QueueHandle_t xQueue);
int xSemaphoreTakeToC(QueueHandle_t xQueue,unsigned int xBlockTime);
QueueHandle_t xQueueCreateMutexToC(char ucQueueType);
void prvInitialiseMutexToC(QueueHandle_t xQueue);