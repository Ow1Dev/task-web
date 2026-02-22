import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable({ providedIn: 'root' })
export class HealthService {
  private apiUrl = 'http://localhost:3000/health';

  constructor(private http: HttpClient) {}

  check() {
    return this.http.get(this.apiUrl);
  }
}
