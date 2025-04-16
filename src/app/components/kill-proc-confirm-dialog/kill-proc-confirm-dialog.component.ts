import {Component, inject} from '@angular/core';
import {AppStore} from "../../app.store";

@Component({
  selector: 'app-kill-proc-confirm-dialog',
  imports: [],
  templateUrl: './kill-proc-confirm-dialog.component.html',
  styleUrl: './kill-proc-confirm-dialog.component.css'
})
export class KillProcConfirmDialogComponent {
  appStore = inject(AppStore)

  cancel() {
    this.appStore.closeConfirmKillProcDialog(false)
  }

  confirm() {
    this.appStore.closeConfirmKillProcDialog(true)
  }
}
