import { Component, OnInit } from '@angular/core';
import { DefaultService } from '../draughts-service-client/api/default.service'

@Component({
  selector: 'app-draughts-ui',
  templateUrl: './draughts-ui.component.html',
  styleUrls: ['./draughts-ui.component.scss']
})
export class DraughtsUiComponent implements OnInit {

  constructor(private draughtsService: DefaultService) {
  }

  ngOnInit(): void {
  }

}
