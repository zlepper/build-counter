import { Injectable } from '@angular/core';
import { LocalStorageService } from './local-storage.service';
import { AuthorizationStateService } from './authorization-state.service';
import { UserService } from './user.service';
import { Observable, of } from 'rxjs';
import { IUser } from '../models';
import { map, switchMap } from 'rxjs/operators';
import { sharingIsCaring } from '../utils';

@Injectable({
  providedIn: 'root',
})
export class AuthorizedUserService {
  public readonly isLoggedIn: Observable<boolean>;

  public readonly loggedInUserInformation: Observable<IUser | null>;

  constructor(
    private authorizationStateService: AuthorizationStateService,
    private localStorageService: LocalStorageService,
    private userService: UserService,
  ) {
    this.authorizationStateService.authToken.subscribe(token => {
      if (token) {
        this.localStorageService.set('authToken', token);
      }
    });

    this.isLoggedIn = this.authorizationStateService.parsedJwt.pipe(
      map(token => {
        if (!token) {
          return false;
        }

        return token.exp >= Date.now() / 1000;
      }),
      sharingIsCaring(),
    );

    this.loggedInUserInformation = this.authorizationStateService.loggedInUserId.pipe(
      switchMap(userId => {
        if (!userId) {
          return of(null);
        }

        return this.userService.getUserInformation(userId);
      }),
    );
  }

  public logout() {
    this.authorizationStateService.authToken.next(null);
    this.localStorageService.remove('authToken');
  }
}
