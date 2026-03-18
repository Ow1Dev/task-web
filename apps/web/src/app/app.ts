import { Component, inject, resource, signal } from '@angular/core';
import { JsonPipe } from '@angular/common';
import { TasksService } from './core';

@Component({
  selector: 'app-root',
  imports: [JsonPipe],
  templateUrl: './app.html',
})
export class App {
  protected readonly title = signal('web');

  #taskService = inject(TasksService);
  readonly tasks = resource({ loader: () => this.#taskService.getTasks() });
}
