import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { DraughtsUiComponent } from './draughts-ui-component/draughts-ui.component';
import {DefaultService } from './draughts-service-client/api/default.service';
import { HttpClientModule } from '@angular/common/http';

@NgModule({
  declarations: [
    AppComponent,
    DraughtsUiComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule
  ],
  providers: [
    DefaultService
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
