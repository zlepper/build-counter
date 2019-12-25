import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DashboardComponent } from './components/dashboard/dashboard.component';
import { ConfiguratorRoutes } from './configurator-routing.module';

@NgModule({
  declarations: [DashboardComponent],
  imports: [CommonModule, ConfiguratorRoutes],
})
export class ConfiguratorModule {}
