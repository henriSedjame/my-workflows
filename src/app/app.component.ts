import {Component, effect, inject, OnDestroy} from '@angular/core';
import {CommonModule} from '@angular/common';
import {Channel, invoke} from "@tauri-apps/api/core";
import {TitleBarComponent} from "./components/title-bar/title-bar.component";
import {AppStore} from "./app.store";
import {EventCallback, EventName, listen, UnlistenFn} from "@tauri-apps/api/event";
import {AppEvents, CommandExecutionEvent, CommandRequested} from "./app.events";
import {CommandViewComponent} from "./components/command-view/command-view.component";
import {KillProcConfirmDialogComponent} from "./components/kill-proc-confirm-dialog/kill-proc-confirm-dialog.component";
import {EXECUTE_COMMAND} from "./app.commands";
import {AppService} from "./app.service";

@Component({
    selector: 'app-root',
    imports: [CommonModule, TitleBarComponent, CommandViewComponent, KillProcConfirmDialogComponent],
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

            this.appService.executeCommand(
                commandId,
                commandScript,
                {label: commandLabel, value: commandValue}
            )
            /*const channel = new Channel<CommandExecutionEvent>()

            channel.onmessage = (message) => {
                switch (message.event) {
                    case AppEvents.COMMAND_STARTED:
                        this.appStore.newCommand(commandId, commandValue, commandLabel, commandScript)
                        break;
                    case AppEvents.COMMAND_PROGRESS:
                        this.appStore.commandInProgress(commandId, message.data.progressLine)
                        break;
                    case AppEvents.COMMAND_FAILED:
                        this.appStore.commandFailed(commandId, message.data.errorsLines)
                        break;
                    case AppEvents.COMMAND_ENDED:
                        this.appStore.commandEnded(commandId, message.data.duration)
                        break;
                }
            }

            invoke<boolean>(EXECUTE_COMMAND, {
                commandId: commandId,
                commandValue: commandScript,
                channel: channel
            })*/
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
