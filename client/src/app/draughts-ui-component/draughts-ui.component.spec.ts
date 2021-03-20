import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { DraughtsUiComponent } from './draughts-ui.component';

describe('DraughtsUiComponent', () => {
  let component: DraughtsUiComponent;
  let fixture: ComponentFixture<DraughtsUiComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ DraughtsUiComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(DraughtsUiComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
