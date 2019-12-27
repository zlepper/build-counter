import { Component, OnInit } from '@angular/core';
import { AuthorizedUserService } from './modules/services';
import { Observable, of } from 'rxjs';
import { getLoginUrl } from './modules/utils';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit {
  public readonly loginUrl: string;

  public isLoggedIn: Observable<boolean> = of(false);

  constructor(private authorizedUserService: AuthorizedUserService) {
    this.loginUrl = getLoginUrl();
  }

  ngOnInit(): void {
    this.isLoggedIn = this.authorizedUserService.isLoggedIn;
  }
}
