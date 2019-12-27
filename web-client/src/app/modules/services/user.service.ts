import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { IUser } from '../models';
import { environment } from '../../../environments/environment';
import { catchError } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class UserService {
  constructor(private httpClient: HttpClient) {}

  public getUserInformation(userId: string): Observable<IUser | null> {
    return this.httpClient.get<IUser>(`${environment.apiServerUrl}/api/user/${userId}`).pipe(
      catchError(err => {
        console.error('Get user failed', err);
        return of(null);
      }),
    );
  }
}
