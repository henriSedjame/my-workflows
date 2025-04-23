import {Component, effect, inject, OnDestroy} from '@angular/core';
import {CommonModule} from '@angular/common';
import {TitleBarComponent} from "./components/title-bar/title-bar.component";
import {AppStore} from "./app.store";
import {EventCallback, EventName, listen, UnlistenFn} from "@tauri-apps/api/event";
import {AppEvents, CommandRequested} from "./app.events";
import {CommandViewComponent} from "./components/command-view/command-view.component";
import {KillProcConfirmDialogComponent} from "./components/kill-proc-confirm-dialog/kill-proc-confirm-dialog.component";
import {AppService} from "./app.service";
import {CommandParamsViewComponent} from "./components/command-params-view/command-params-view.component";

@Component({
    selector: 'app-root',
    imports: [CommonModule, TitleBarComponent, CommandViewComponent, KillProcConfirmDialogComponent, CommandParamsViewComponent],
    templateUrl: './app.component.html',
    styleUrl: './app.component.css'
})
export class AppComponent implements OnDestroy {
    appStore = inject(AppStore)
    appService = inject(AppService)
    unListens: UnlistenFn[] = []

    _e = effect(async () => {
        await this.addListener(AppEvents.APP_FOCUSED, () => this.appStore.setFocused(true))
        await this.addListener(AppEvents.APP_BLUR, () => this.appStore.setFocused(false))
        await this.addListener<CommandRequested>(AppEvents.CMD_REQUESTED, (event) => {

            const commandId = event.payload.commandId
            const commandLabel = event.payload.commandLabel
            const commandValue = event.payload.commandValue
            const commandScript = event.payload.commandToExecute

            let commandParams = event.payload.commandParams;

            this.appService.executeCommand(
                commandId,
                commandScript,
                {label: commandLabel, value: commandValue, parameters: commandParams},
            )

        })
    })

    ngOnDestroy(): void {
        this.unListens.forEach(fn => fn())
    }

    async addListener<T>(event: EventName, handler: EventCallback<T>): Promise<void> {
        this.unListens = [
            ...this.unListens,
            await listen<T>(event, handler)
        ]
    }
}
