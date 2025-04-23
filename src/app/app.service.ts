import {inject, Injectable} from "@angular/core";
import {Channel, invoke} from "@tauri-apps/api/core";
import {EXECUTE_COMMAND} from "./app.commands";
import {AppEvents, CommandExecutionEvent} from "./app.events";
import {AppStore} from "./app.store";

@Injectable(
    {providedIn: 'root'}
)
export class AppService {

    appStore = inject(AppStore)

    executeCommand(commandId: string, script: string, startCommandData:{label: string, value: string} | undefined = undefined) {

        const channel = new Channel<CommandExecutionEvent>()

        channel.onmessage = (message) => {
            switch (message.event) {
                case AppEvents.COMMAND_STARTED:
                    if (startCommandData == undefined) {
                        this.appStore.replayCommand(commandId)
                    } else {
                        this.appStore.newCommand(commandId, startCommandData.value, startCommandData.label, script)
                    }

                    break;
                case AppEvents.COMMAND_PROGRESS:
                    this.appStore.commandInProgress(commandId, message.data.progressLine)
                    break;
                case AppEvents.COMMAND_FAILED:
                    this.appStore.commandFailed(commandId, message.data.errorsLines, message.data.duration)
                    break;
                case AppEvents.COMMAND_ENDED:
                    this.appStore.commandEnded(commandId, message.data.duration, message.data.statusCode)
                    break;
            }
        }

        invoke<boolean>(EXECUTE_COMMAND, {
            commandId: commandId,
            commandValue: script,
            channel: channel
        })
    }

}