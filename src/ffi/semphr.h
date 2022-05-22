#include "ffi.h"
#include "queue.h"

#define xSemaphoreCreateBinary xSemaphoreCreateBinaryToC
#define xSemaphoreCreateCounting xSemaphoreCreateCountingToC
#define vSemaphoreDelete vSemaphoreDeleteToC
#define xSemaphoreGive xSemaphoreGiveToC
#define xSemaphoreTake xSemaphoreTakeToC
#define xQueueCreateMutex xQueueCreateMutexToC
#define prvInitialiseMutex prvInitialiseMutexToC

QueueHandle_t xSemaphoreCreateBinaryToC();
QueueHandle_t xSemaphoreCreateCountingToC(unsigned int uxMaxCount,unsigned int uxInitialCount);
void vSemaphoreDeleteToC(QueueHandle_t xQueue);
int xSemaphoreGiveToC(QueueHandle_t xQueue);
int xSemaphoreTakeToC(QueueHandle_t xQueue,unsigned int xBlockTime);
QueueHandle_t xQueueCreateMutexToC(char ucQueueType);
void prvInitialiseMutexToC(QueueHandle_t xQueue);