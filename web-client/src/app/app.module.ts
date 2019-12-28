import { BrowserModule } from '@angular/platform-browser';
import { APP_INITIALIZER, NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatButtonModule, MatToolbarModule } from '@angular/material';
import { AuthorizationStateService, LocalStorageService } from './modules/services';
import { HTTP_INTERCEPTORS, HttpClientModule } from '@angular/common/http';
import { AuthInterceptor } from './modules/services/auth.interceptor';

const TOKEN_KEY = 'token';

function startup(authorizationStateService: AuthorizationStateService, localStorageService: LocalStorageService) {
  return () => {
    const query = new URLSearchParams(location.search);
    if (query.has(TOKEN_KEY)) {
      const token = query.get(TOKEN_KEY);
      authorizationStateService.authToken.next(token);

      query.delete(TOKEN_KEY);

      const a = document.createElement('a');
      a.href = location.href;
      a.search = query.toString();

      history.replaceState(null, '', a.href);
    } else {
      const existingAuthToken = localStorageService.get('authToken');
      if (existingAuthToken !== null) {
        authorizationStateService.authToken.next(existingAuthToken);
      }
    }
  };
}

@NgModule({
  declarations: [AppComponent],
  imports: [BrowserModule, AppRoutingModule, BrowserAnimationsModule, MatToolbarModule, MatButtonModule, HttpClientModule],
  providers: [
    {
      provide: HTTP_INTERCEPTORS,
      multi: true,
      useClass: AuthInterceptor,
    },
    {
      provide: APP_INITIALIZER,
      multi: true,
      useFactory: startup,
      deps: [AuthorizationStateService, LocalStorageService],
    },
  ],
  bootstrap: [AppComponent],
})
export class AppModule {}
