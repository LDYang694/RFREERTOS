#include "ffi.h"

#define QueueHandle_t void *
#define xQueueCreate xQueueCreateToC
#define xQueueSend xQueueSendToC
#define xQueueReceive xQueueReceiveToC
#define xQueuePeek xQueuePeekToC
#define vQueueDelete vQueueDeleteToC
#define xQueueSendFromISR xQueueSendFromISRToC
#define xQueueReceiveFromISR xQueueReceiveFromISRToC
#define xQueuePeekFromISR xQueuePeekFromISRToC
#define vQueueDeleteFromISR vQueueDeleteFromISRToC
#define xQueueReset xQueueResetToC
#define uxQueueMessagesWaitingFromISR uxQueueMessagesWaiting

QueueHandle_t xQueueCreateToC(unsigned int uxQueueLength,unsigned int uxItemSize);
int uxQueueMessagesWaiting(QueueHandle_t xQueue);
int uxQueueSpacesAvailable(QueueHandle_t xQueue);
bool xQueueIsQueueEmptyFromISR(QueueHandle_t xQueue);
int8_t cGetQueueRxLock(QueueHandle_t xQueue);
int8_t cGetQueueTxLock(QueueHandle_t xQueue);
void vSetQueueRxLock(QueueHandle_t xQueue,int8_t RxLock);
void vSetQueueTxLock(QueueHandle_t xQueue,int8_t TxLock);
int xQueueSendToC(QueueHandle_t xQueue,void* pvItemToQueue,int xTicksToWait);
int xQueueReceiveToC(QueueHandle_t xQueue,void* pvItemToQueue,int xTicksToWait);
int xQueuePeekToC(QueueHandle_t xQueue,void* pvItemToQueue,int xTicksToWait);
void vQueueDeleteToC(QueueHandle_t xQueue);
int xQueueSendFromISRToC(QueueHandle_t xQueue,void* pvItemToQueue,int *pxHigherPriorityTaskWoken);
int xQueueReceiveFromISRToC(QueueHandle_t xQueue,void* pvItemToQueue,int *pxHigherPriorityTaskWoken);
int xQueuePeekFromISRToC(QueueHandle_t xQueue,void* pvItemToQueue);
int xQueueResetToC(QueueHandle_t xQueue);