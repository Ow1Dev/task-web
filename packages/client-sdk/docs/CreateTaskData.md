
# CreateTaskData


## Properties

Name | Type
------------ | -------------
`description` | string
`title` | string

## Example

```typescript
import type { CreateTaskData } from '@task-web/client-sdk'

// TODO: Update the object below with actual values
const example = {
  "description": null,
  "title": null,
} satisfies CreateTaskData

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as CreateTaskData
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


