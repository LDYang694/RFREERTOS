#include "ffi.h"

#define QueueHandle_t void *
#define xQueueCreate xQueueCreateToC
#define xQueueSend xQueueSendToC
#define xQueueReceive xQueueReceiveToC
#define xQueuePeek xQueuePeekToC
#define vQueueDelete vQueueDeleteToC
#define xQueueReceiveFromISR xQueueReceiveFromISRToC
#define xQueuePeekFromISR xQueuePeekFromISRToC
#define vQueueDeleteFromISR vQueueDeleteFromISRToC

QueueHandle_t xQueueCreateToC(unsigned int uxQueueLength,unsigned int uxItemSize);
int uxQueueMessagesWaiting(QueueHandle_t xQueue);
char cGetQueueRxLock(QueueHandle_t xQueue);
char cGetQueueTxLock(QueueHandle_t xQueue);
int xQueueSendToC(QueueHandle_t xQueue,void* pvItemToQueue,int xTicksToWait);
int xQueueReceiveToC(QueueHandle_t xQueue,void* pvItemToQueue,int xTicksToWait);
int xQueuePeekToC(QueueHandle_t xQueue,void* pvItemToQueue,int xTicksToWait);
void vQueueDeleteToC(QueueHandle_t xQueue);
int xQueueSendFromISRToC(QueueHandle_t xQueue,void* pvItemToQueue,int *pxHigherPriorityTaskWoken);
int xQueueReceiveFromISRToC(QueueHandle_t xQueue,void* pvItemToQueue,int *pxHigherPriorityTaskWoken);
int xQueuePeekFromISRToC(QueueHandle_t xQueue,void* pvItemToQueue);