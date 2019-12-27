import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { map } from 'rxjs/operators';
import { parseJwt } from '../utils';

@Injectable({
  providedIn: 'root',
})
export class AuthorizationStateService {
  public readonly authToken = new BehaviorSubject<string | null>(null);

  public readonly parsedJwt = this.authToken.pipe(
    map(token => {
      if (!token) {
        return null;
      }

      return parseJwt<{ sub: string; exp: number }>(token);
    }),
  );

  public readonly loggedInUserId = this.parsedJwt.pipe(
    map(token => {
      if (!token) {
        return null;
      }

      return token.sub;
    }),
  );
}
