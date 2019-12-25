import { RouterModule, Routes } from '@angular/router';
import { FrontPageComponent } from './components/front-page/front-page.component';

const routes: Routes = [
  {
    path: '',
    component: FrontPageComponent,
  },
];

export const HomePageRoutes = RouterModule.forChild(routes);
