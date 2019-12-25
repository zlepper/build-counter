import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FrontPageComponent } from './components/front-page/front-page.component';
import { HomePageRoutes } from './home-page-routing.module';

@NgModule({
  declarations: [FrontPageComponent],
  imports: [CommonModule, HomePageRoutes],
})
export class HomePageModule {}
