#include "ffi.h"

#define TaskHandle_t void*

#define tskIDLE_PRIORITY    ( ( UBaseType_t ) 0U )

#define xTaskCreateStatic xTaskCreateStaticToC
#define xTaskCreate xTaskCreateToC
#define vTaskSuspend vTaskSuspendToC
#define vTaskResume vTaskResumeToC
#define taskENTER_CRITICAL taskENTER_CRITICAL_ToC
#define taskEXIT_CRITICAL taskEXIT_CRITICAL_ToC
#define xTaskGetTickCount xTaskGetTickCountToC
#define vTaskDelayUntil xTaskDelayUntil

void vTaskStartScheduler();
void* get_task_handle();

TaskHandle_t xTaskCreateStaticToC(unsigned int pxTaskCode,char* pcName,unsigned int ulStackDepth,void* pvParameters,
        void* puxStackBuffer,TaskHandle_t pxTaskBuffer,unsigned int uxPriority);

void xTaskCreateToC(unsigned int pxTaskCode,char* pcName,unsigned int ulStackDepth,void* pvParameters,
        unsigned int uxPriority,TaskHandle_t pxTaskBuffer);

void vTaskSuspendToC(TaskHandle_t xTaskToSuspend_);
void vTaskResumeToC(TaskHandle_t xTaskToResume_);

void taskENTER_CRITICAL_ToC();
void taskEXIT_CRITICAL_ToC();

void vTaskDelay(TickType_t xTicksToDelay);
void xTaskDelayUntil(TickType_t *pxPreviousWakeTime, TickType_t xTimeIncrement);
void vTaskSuspendAll();
void vTaskResumeAll();

int xTaskGetTickCountToC();

typedef struct xTIME_OUT
{
    BaseType_t xOverflowCount;
    TickType_t xTimeOnEntering;
} TimeOut_t;