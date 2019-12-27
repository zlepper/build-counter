import { Component, OnInit } from '@angular/core';
import { AuthorizedUserService } from './modules/services';
import { Observable, of } from 'rxjs';
import { getLoginUrl } from './modules/utils';
import { IUser } from './modules/models';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit {
  public readonly loginUrl: string;

  public isLoggedIn: Observable<boolean> = of(false);

  public userInformation: Observable<IUser | null> = of(null);

  constructor(private authorizedUserService: AuthorizedUserService) {
    this.loginUrl = getLoginUrl();
  }

  ngOnInit(): void {
    this.isLoggedIn = this.authorizedUserService.isLoggedIn;
    this.userInformation = this.authorizedUserService.loggedInUserInformation;
  }
}
