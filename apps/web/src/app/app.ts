import { Component, inject, signal } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import { RouterOutlet } from '@angular/router';
import { HealthService } from './core/services/health.service';
import { catchError, map, of } from 'rxjs';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.html',
})
export class App {
  protected readonly title = signal('web');

  apiStatus = toSignal(
    inject(HealthService)
      .check()
      .pipe(
        map((res: any) => (res.status === 'Ok' ? 'up' : 'down')),
        catchError(() => of('down')),
      ),
    { initialValue: 'checking' },
  );
}
