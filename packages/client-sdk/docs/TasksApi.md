# TasksApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createTasks**](TasksApi.md#createtasks) | **POST** /v1/tasks |  |
| [**deleteTasks**](TasksApi.md#deletetasks) | **DELETE** /v1/tasks/{id} |  |
| [**getTasks**](TasksApi.md#gettasks) | **GET** /v1/tasks |  |
| [**updateTasks**](TasksApi.md#updatetasks) | **PATCH** /v1/tasks/{id} |  |



## createTasks

> TaskResponse createTasks(createTaskData)



### Example

```ts
import {
  Configuration,
  TasksApi,
} from '@task-web/client-sdk';
import type { CreateTasksRequest } from '@task-web/client-sdk';

async function example() {
  console.log("🚀 Testing @task-web/client-sdk SDK...");
  const api = new TasksApi();

  const body = {
    // CreateTaskData
    createTaskData: ...,
  } satisfies CreateTasksRequest;

  try {
    const data = await api.createTasks(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **createTaskData** | [CreateTaskData](CreateTaskData.md) |  | |

### Return type

[**TaskResponse**](TaskResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Task created |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## deleteTasks

> deleteTasks(id)



### Example

```ts
import {
  Configuration,
  TasksApi,
} from '@task-web/client-sdk';
import type { DeleteTasksRequest } from '@task-web/client-sdk';

async function example() {
  console.log("🚀 Testing @task-web/client-sdk SDK...");
  const api = new TasksApi();

  const body = {
    // number | Task ID
    id: 56,
  } satisfies DeleteTasksRequest;

  try {
    const data = await api.deleteTasks(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | `number` | Task ID | [Defaults to `undefined`] |

### Return type

`void` (Empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **204** | Task deleted |  -  |
| **404** | Task not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getTasks

> Array&lt;TaskResponse&gt; getTasks()



### Example

```ts
import {
  Configuration,
  TasksApi,
} from '@task-web/client-sdk';
import type { GetTasksRequest } from '@task-web/client-sdk';

async function example() {
  console.log("🚀 Testing @task-web/client-sdk SDK...");
  const api = new TasksApi();

  try {
    const data = await api.getTasks();
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**Array&lt;TaskResponse&gt;**](TaskResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Get all tasks |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## updateTasks

> TaskResponse updateTasks(id, updateTaskData)



### Example

```ts
import {
  Configuration,
  TasksApi,
} from '@task-web/client-sdk';
import type { UpdateTasksRequest } from '@task-web/client-sdk';

async function example() {
  console.log("🚀 Testing @task-web/client-sdk SDK...");
  const api = new TasksApi();

  const body = {
    // number | Task ID
    id: 56,
    // UpdateTaskData
    updateTaskData: ...,
  } satisfies UpdateTasksRequest;

  try {
    const data = await api.updateTasks(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | `number` | Task ID | [Defaults to `undefined`] |
| **updateTaskData** | [UpdateTaskData](UpdateTaskData.md) |  | |

### Return type

[**TaskResponse**](TaskResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Task updated |  -  |
| **404** | Task not found |  -  |
| **500** | Internal server error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

