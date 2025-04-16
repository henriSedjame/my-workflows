import {Component, effect, inject, OnDestroy, signal} from '@angular/core';
import {CommonModule} from '@angular/common';
import {Channel, invoke} from "@tauri-apps/api/core";
import {TitleBarComponent} from "./components/title-bar/title-bar.component";
import {AppStore} from "./app.store";
import {EventCallback, EventName, listen, UnlistenFn} from "@tauri-apps/api/event";
import {AppEvents, CommandExecutionEvent, CommandRequested} from "./app.events";
import {CommandViewComponent} from "./components/command-view/command-view.component";
import {KillProcConfirmDialogComponent} from "./components/kill-proc-confirm-dialog/kill-proc-confirm-dialog.component";

@Component({
    selector: 'app-root',
    imports: [CommonModule, TitleBarComponent, CommandViewComponent, KillProcConfirmDialogComponent],
    templateUrl: './app.component.html',
    styleUrl: './app.component.css'
})
export class AppComponent implements OnDestroy {
    appStore = inject(AppStore)
    unListens: UnlistenFn[] = []

    _e = effect(async () => {
        await this.addListener(AppEvents.APP_FOCUSED, () => this.appStore.setFocused(true))
        await this.addListener(AppEvents.APP_BLUR, () => this.appStore.setFocused(false))
        await this.addListener<CommandRequested>(AppEvents.CMD_REQUESTED, (event) => {

            const commandId = event.payload.commandId
            const commandLabel = event.payload.commandLabel
            const commandValue = event.payload.commandValue

            const channel = new Channel<CommandExecutionEvent>()

            channel.onmessage = (message) => {
                switch (message.event) {
                    case 'commandStarted':
                        this.appStore.newCommand(commandId, commandValue, commandLabel)
                        break;
                    case 'commandProgress':
                        this.appStore.commandInProgress(commandId, message.data.progressLine)
                        break;
                    case 'commandFailed':
                        this.appStore.commandFailed(commandId, message.data.errorsLines)
                        break;
                    case 'commandEnded':
                        this.appStore.commandEnded(commandId, message.data.duration)
                        break;
                }

                console.log(message)
            }

            invoke<boolean>('execute_command', {
                commandId: commandId,
                commandValue: event.payload.commandValue,
                channel: channel
            })
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
