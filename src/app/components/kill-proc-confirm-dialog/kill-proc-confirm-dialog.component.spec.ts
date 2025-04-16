import { ComponentFixture, TestBed } from '@angular/core/testing';

import { KillProcConfirmDialogComponent } from './kill-proc-confirm-dialog.component';

describe('KillProcConfirmDialogComponent', () => {
  let component: KillProcConfirmDialogComponent;
  let fixture: ComponentFixture<KillProcConfirmDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [KillProcConfirmDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(KillProcConfirmDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
