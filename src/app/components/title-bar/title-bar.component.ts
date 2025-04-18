import {Component, computed, inject} from '@angular/core';
import {AppStore} from "../../app.store";
import {NgClass} from "@angular/common";
import {TabHeaderComponent} from "../tab-header/tab-header.component";
import {invoke} from "@tauri-apps/api/core";
import {HIDE_VIEW} from "../../app.commands";


@Component({
    selector: 'app-title-bar',
    imports: [
        TabHeaderComponent,
        NgClass
    ],
    templateUrl: './title-bar.component.html',
    styleUrl: './title-bar.component.css'
})
export class TitleBarComponent {

    appStore = inject(AppStore);

    activeBtnClass = computed(() => ({
        'active': this.appStore.focused()
    }))
    
    hideView() {
        invoke(HIDE_VIEW, { openTabs: true }).then()
    }

}
