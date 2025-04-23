import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CommandParamsViewComponent } from './command-params-view.component';

describe('CommandParamsViewComponent', () => {
  let component: CommandParamsViewComponent;
  let fixture: ComponentFixture<CommandParamsViewComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CommandParamsViewComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CommandParamsViewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
