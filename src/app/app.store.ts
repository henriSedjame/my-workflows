import {patchState, signalStore, withComputed, withMethods, withState} from "@ngrx/signals";
import {computed} from "@angular/core";
import {invoke} from "@tauri-apps/api/core";
import {HIDE_VIEW, KILL_COMMAND} from "./app.commands";


export type CommandStatus = 'started' | 'in_progress' | 'succeeded' | 'failed' | 'cancelled';

type Command = {
    id: string,
    label: string,
    value: string,
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

type KillProcDialogData = {
    commandId: string,
    closeTab: boolean,
}

type AppState = {
    focused: boolean;
    commands: Command[],
    currentTab: string|undefined,
    killProcDialogData: KillProcDialogData | undefined,
}

export const initialState: AppState = {
    focused: false,
    commands: [],
    currentTab: undefined,
    killProcDialogData: undefined,
}

export const AppStore = signalStore(
    {providedIn: 'root'},
    withState<AppState>(initialState),
    withComputed((state) => ({
        tabs: computed<Tab[]>(() => state.commands().map((command) => ({id: command.id, label: command.label, status: command.status})),),
        currentCommand: computed(() => {
            const tab = state.currentTab()

            if (tab) {
                return state.commands().filter((command: Command) => command.id == tab)[0]
            }

            return undefined
        }),
        killProcDialogOpened: computed(() => state.killProcDialogData() != undefined)
    })),
    withMethods((store) => ({

        setFocused(value: boolean) {
            patchState(store, {focused: value})
        },

        newCommand(id: string, value: string, label: string) {

            const commands = store.commands()

            patchState(store, {
                commands: [
                    ...commands,
                    {
                        id: id,
                        label: label,
                        value: value,
                        progressLines: [],
                        errorLines: [],
                        status: 'started',
                        duration: undefined
                    }
                ],
                currentTab: id
            })
        },

        commandInProgress(id: string, line: string) {
            patchState(store, {
                commands: this._updateCommands(
                    store.commands(),
                    id,
                    (command) => ({
                        ...command,
                        status: 'in_progress',
                        progressLines: [
                            ...command.progressLines,
                            line
                        ]
                    })
                ),
            })
        },

        commandFailed(id: string, lines: string[]) {
            patchState(store, {
                commands: this._updateCommands(
                    store.commands(),
                    id,
                    (command) => ({
                        ...command,
                        status: 'failed',
                        errorLines: lines
                    })
                ),
            })
        },

        commandEnded(id: string, duration: number) {
            patchState(store, {
                commands: this._updateCommands(
                    store.commands(),
                    id,
                    (command) => ({
                        ...command,
                        status: 'succeeded',
                        duration: duration
                    })
                ),
            })
        },

        commandCancelled(id: string) {
            patchState(store, {
                commands: this._updateCommands(
                    store.commands(),
                    id,
                    (command) => ({
                        ...command,
                        status: 'cancelled'
                    })
                ),
            })
        },

        setCurrentTab(tab: Tab) {
            patchState(store, {currentTab: tab.id})
        },

        closeTab(id: string) {
            const cmds = store.commands().filter((cmd: Command) => cmd.id != id)

            let tab = store.currentTab()

            if (tab && id == tab && cmds.length > 0) {
                const cmd = cmds[0]
                tab = cmd.id
            }

            patchState(store, {
                commands: cmds,
                currentTab: tab,
            })

            if (cmds.length === 0) {
                invoke(HIDE_VIEW, { openTabs: false }).then()
            }

        },

        openConfirmKillProcDialog(id: string, closeTab: boolean) {
           patchState(store, {
               killProcDialogData: { commandId: id, closeTab: closeTab },
           })
        },

        closeConfirmKillProcDialog(killProc: boolean) {
            if (killProc) {
                const data = store.killProcDialogData()
                if(data) {
                    invoke<boolean>(KILL_COMMAND, {
                        commandId: data.commandId
                    }).then(killed => {
                        if (killed) {
                            this.commandCancelled(data.commandId)
                            if (data.closeTab) {
                                this.closeTab(data.commandId)
                            }
                        }
                    });
                }
            }
            patchState(store, {
                killProcDialogData: undefined
            })
        },

        _updateCommands(commands: Command[], id: string, buildCommand: (arg0: Command) => Command): Command[] {
            return commands.map(command => {
                if (command.id == id) {
                    return buildCommand(command)
                } else {
                    return command
                }
            })
        }

    }))
)