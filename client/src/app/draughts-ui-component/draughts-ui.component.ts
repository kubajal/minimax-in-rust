import { Component, OnInit } from '@angular/core';
import { DefaultService } from '../draughts-service-client/api/default.service'
import { Board } from '../draughts-service-client/model/board';

@Component({
  selector: 'app-draughts-ui',
  templateUrl: './draughts-ui.component.html',
  styleUrls: ['./draughts-ui.component.scss']
})
export class DraughtsUiComponent implements OnInit {

  constructor(private draughtsService: DefaultService) {
  }

  board = new Board(8);

  ngOnInit(): void {

  }

}
