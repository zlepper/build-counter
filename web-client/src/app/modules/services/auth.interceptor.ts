import { HttpEvent, HttpHandler, HttpInterceptor, HttpRequest } from '@angular/common/http';
import { Observable } from 'rxjs';
import { AuthorizationStateService } from './authorization-state.service';
import { environment } from '../../../environments/environment';
import { switchMap } from 'rxjs/operators';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root',
})
export class AuthInterceptor implements HttpInterceptor {
  constructor(private authorizationStateService: AuthorizationStateService) {}

  intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    if (!req.url.startsWith(environment.apiServerUrl)) {
      return next.handle(req);
    }

    return this.authorizationStateService.authToken.pipe(
      switchMap(token => {
        if (!token) {
          return next.handle(req);
        }

        return next.handle(
          req.clone({
            setHeaders: {
              Authorization: `Bearer ${token}`,
            },
          }),
        );
      }),
    );
  }
}
