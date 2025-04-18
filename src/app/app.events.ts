export enum AppEvents {
    APP_FOCUSED = 'tauri://focus',
    APP_BLUR = 'tauri://blur',
    CMD_REQUESTED = 'COMMAND_REQUESTED',
    COMMAND_STARTED = 'commandStarted',
    COMMAND_PROGRESS = 'commandProgress',
    COMMAND_FAILED = 'commandFailed',
    COMMAND_ENDED = 'commandEnded'
}

export type CommandRequested = {
    commandId: string,
    commandLabel: string,
    commandValue: string,
    commandToExecute: string,
}

export type CommandExecutionEvent =
    { event: AppEvents.COMMAND_STARTED }
    | { event: AppEvents.COMMAND_PROGRESS, data: { progressLine: string } }
    | { event: AppEvents.COMMAND_FAILED, data: { errorsLines: string[] } }
    | { event: AppEvents.COMMAND_ENDED, data: { duration: number } }