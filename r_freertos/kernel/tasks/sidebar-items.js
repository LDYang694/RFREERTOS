initSidebarItems({"constant":[["taskEVENT_LIST_ITEM_VALUE_IN_USE",""]],"enum":[["eTaskState",""]],"fn":[["TCB_set_pxStack","set pxStack of target tcb"],["prvAddCurrentTaskToDelayedList","add current task to delayed list"],["prvAddNewTaskToReadyList","add task to ready list"],["prvAddTaskToReadyList","add task to ready list"],["prvGetTCBFromHandle","get tcb from handle, return current tcb if handle is None"],["prvIdleTask","idle task function"],["prvInitialiseNewTask","initialise new task"],["prvInitialiseTaskLists",""],["prvResetNextTaskUnblockTime","reset NextTaskUnblockTime"],["prvTaskIsTaskSuspended",""],["pvTaskIncrementMutexHeldCount",""],["pxPortInitialiseStack",""],["taskRECORD_READY_PRIORITY",""],["taskSELECT_HIGHEST_PRIORITY","find highest priority with valid task"],["taskSELECT_HIGHEST_PRIORITY_TASK","set current tcb to task with highest priority"],["taskSWITCH_DELAYED_LISTS","in case of mtime overflow, swap delayed list and overflow list"],["taskYield","yield in task"],["uxTaskPriorityGet","get priority of target task"],["vTaskDelay",""],["vTaskDelete","delete target task"],["vTaskEnterCritical",""],["vTaskExitCritical",""],["vTaskInternalSetTimeOutState","set pxTimeOut to current time (in kernal)"],["vTaskPlaceOnEventList",""],["vTaskPriorityDisinheritAfterTimeout",""],["vTaskPrioritySet","set target task’s priority"],["vTaskRemoveFromUnorderedEventList",""],["vTaskResume","resume target task"],["vTaskResumeAll","resume scheduler"],["vTaskSetTimeOutState","set pxTimeOut to current time (in task)"],["vTaskStartScheduler","start scheduler"],["vTaskSuspend","suspend task until resumed"],["vTaskSuspendAll","suspend scheduler"],["xPortSysTickHandler",""],["xTaskCheckForTimeOut","return if timeout has been reached"],["xTaskCreate","create task (not static)"],["xTaskCreateStatic","create task (static)"],["xTaskDelayUntil","delay task until pxPreviousWakeTime+pxPreviousWakeTime "],["xTaskIncrementTick","tick increment, free delayed task"],["xTaskPriorityDisinherit",""],["xTaskPriorityInherit",""],["xTaskRemoveFromEventList","remove first task from event list, and insert the task to ready list"]],"static":[["XSCHEDULERRUNNING",""],["tskIDLE_PRIORITY",""],["uxCurrentNumberOfTasks",""],["uxSchedulerSuspended",""],["xNextTaskUnblockTime",""],["xNumOfOverflows",""],["xPendedTicks",""],["xSchedulerRunning",""],["xTickCount",""],["xYieldPending",""]],"struct":[["TimeOut",""],["tskTaskControlBlock",""]],"type":[["Param_link",""],["StackType_t",""],["StackType_t_link",""],["TCB_t",""],["TCB_t_link",""],["TaskFunction_t",""],["TaskHandle_t",""],["UBaseType_t",""],["tskTCB",""]]});