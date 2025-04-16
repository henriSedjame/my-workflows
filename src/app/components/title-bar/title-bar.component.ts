import {Component, computed, inject} from '@angular/core';
import {AppStore} from "../../app.store";
import {NgClass} from "@angular/common";
import {getCurrentWindow} from '@tauri-apps/api/window';
import {TabHeaderComponent} from "../tab-header/tab-header.component";


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

    appWindow = getCurrentWindow()

    appStore = inject(AppStore);

    activeBtnClass = computed(() => ({
        'active': this.appStore.focused()
    }))

}
