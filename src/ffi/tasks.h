#include "ffi.h"

#define TaskHandle_t void*

#define xTaskCreateStatic xTaskCreateStaticToC
#define xTaskCreate xTaskCreateToC
#define vTaskSuspend vTaskSuspendToC
#define vTaskResume vTaskResumeToC

void vTaskStartScheduler();
void* get_task_handle();

TaskHandle_t xTaskCreateStaticToC(unsigned int pxTaskCode,char* pcName,unsigned int ulStackDepth,void* pvParameters,
        void* puxStackBuffer,TaskHandle_t pxTaskBuffer,unsigned int uxPriority);

void xTaskCreateToC(unsigned int pxTaskCode,char* pcName,unsigned int ulStackDepth,void* pvParameters,
        unsigned int uxPriority,TaskHandle_t pxTaskBuffer);

void vTaskSuspendToC(TaskHandle_t xTaskToSuspend_);
void vTaskResumeToC(TaskHandle_t xTaskToResume_);