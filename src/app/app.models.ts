export type CommandStatus = 'started' | 'in_progress' | 'succeeded' | 'failed' | 'cancelled';

export type Command = {
    id: string,
    label: string,
    value: string,
    executedScript: string,
    progressLines: string[];
    errorLines: string[];
    status: CommandStatus;
    duration: number | undefined;
}

export type Tab = {
    id: string,
    label: string,
    status: CommandStatus,
}

export type KillProcDialogData = {
    commandId: string,
    closeTab: boolean,
}

