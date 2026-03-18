// tasks.service.ts
import { Injectable } from '@angular/core';
import { TasksApi, Configuration, CreateTaskData, UpdateTaskData } from '@task-web/client-sdk';

@Injectable({ providedIn: 'root' })
export class TasksService {
  private api = new TasksApi(
    new Configuration({
      basePath: 'http://localhost:3000',
    }),
  );

  getTasks() {
    return this.api.getTasks();
  }

  createTask(data: CreateTaskData) {
    return this.api.createTasks({ createTaskData: data });
  }

  updateTask(id: number, data: UpdateTaskData) {
    return this.api.updateTasks({ id, updateTaskData: data });
  }

  deleteTask(id: number) {
    return this.api.deleteTasks({ id });
  }
}
