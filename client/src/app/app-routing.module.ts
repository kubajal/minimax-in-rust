import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { DraughtsUiComponent } from './draughts-ui-component/draughts-ui.component';

const routes: Routes = [
  { path: '', component: DraughtsUiComponent }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
