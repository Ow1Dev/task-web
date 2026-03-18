import { Component, inject, resource, signal } from '@angular/core';
import { JsonPipe } from '@angular/common';
import { toSignal } from '@angular/core/rxjs-interop';
import { RouterOutlet } from '@angular/router';
import { TasksService } from './core';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, JsonPipe],
  templateUrl: './app.html',
})
export class App {
  protected readonly title = signal('web');

  #taskService = inject(TasksService);
  readonly tasks = resource({ loader: () => this.#taskService.getTasks() });
}
