import {Component, computed, inject, input} from '@angular/core';
import {AppStore, Tab} from "../../app.store";
import {NgClass} from "@angular/common";

@Component({
  selector: 'app-tab-header',
  imports: [
    NgClass
  ],
  templateUrl: './tab-header.component.html',
  styleUrl: './tab-header.component.css'
})
export class TabHeaderComponent {
  tab = input.required<Tab>()
  appStore = inject(AppStore);


  activeTabClass = computed(() => ({
    'active-tab': this.appStore.currentTab() == this.tab().id,
    'inactive-tab': this.appStore.currentTab() != this.tab().id
  }))

  closeTab(event: MouseEvent) {
    event.stopPropagation();

    if (this.tab().status === 'in_progress') {
      this.appStore.openConfirmKillProcDialog(this.tab().id, true);
    } else {
      this.appStore.closeTab(this.tab().id)
    }
  }

}
