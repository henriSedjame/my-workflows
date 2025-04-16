export enum AppEvents {
    APP_FOCUSED = 'tauri://focus',
    APP_BLUR = 'tauri://blur',
    CMD_REQUESTED = 'COMMAND_REQUESTED'
}

export type CommandRequested = {
    commandId: string,
    commandLabel: string,
    commandValue: string,
}

export type CommandExecutionEvent =
    { event: 'commandStarted' }
    | { event: 'commandProgress', data: { progressLine: string } }
    | { event: 'commandFailed', data: { errorsLines: string[] } }
    | { event: 'commandEnded', data: { duration: number } }