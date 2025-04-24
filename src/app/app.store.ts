import {patchState, signalStore, withComputed, withMethods, withState} from "@ngrx/signals";
import {computed} from "@angular/core";
import {invoke} from "@tauri-apps/api/core";
import {HIDE_VIEW, KILL_COMMAND} from "./app.commands";
import {Command, CommandParam, KillProcDialogData, Tab} from "./app.models";


type AppState = {
    focused: boolean;
    commands: Command[],
    currentTab: string | undefined,
    killProcDialogData: KillProcDialogData | undefined,
    nextCommandParams: CommandParam[] | undefined,
}

export const initialState: AppState = {
    focused: false,
    commands: [],
    currentTab: undefined,
    killProcDialogData: undefined,
    nextCommandParams: undefined,
}

export const AppStore = signalStore(
    {providedIn: 'root'},
    withState<AppState>(initialState),
    withComputed((state) => ({
        tabs: computed<Tab[]>(() => state.commands().map((command) => ({
            id: command.id,
            label: command.label,
            status: command.status
        })),),
        currentCommand: computed(() => {
            const tab = state.currentTab()

            if (tab) {
                return state.commands().filter((command: Command) => command.id == tab)[0]
            }

            return undefined
        }),
        killProcDialogOpened: computed(() => state.killProcDialogData() != undefined),
        showParamsView: computed(() => state.nextCommandParams() != undefined),
    })),
    withMethods((store) => ({

        setFocused(value: boolean) {
            patchState(store, {focused: value})
        },

        requestNextCommandParams(id: string, value: string, label: string, script: string, params: string[]) {
            patchState(store, {
                commands: [
                    ...store.commands(),
                    {
                        id: id,
                        label: label,
                        value: value,
                        executedScript: script,
                        progressLines: [],
                        errorLines: [],
                        status: 'waiting_for_params',
                        duration: undefined
                    }
                ],
                nextCommandParams: params.map((param) => ({
                    name: param,
                    value: undefined
                })),
                currentTab: id
            })
        },

        setNextCommandParamValue(paramName: string, value: string) {
            patchState(store, {
                nextCommandParams: store.nextCommandParams()?.map((param) => {
                    if (param.name == paramName) {
                        return {
                            ...param,
                            value: value
                        }
                    }
                    return param
                })
            })
        },

        cancelNextCommandParams() {
            const id = store.currentTab()
            patchState(store, {
                nextCommandParams: undefined,
                currentTab: undefined
            })
            if (id) {
                this.closeTab(id)
            }
        },

        newCommand(id: string, value: string, label: string, script: string) {

            const commands = store.commands()

            let newCommands: Command[] = [];

            if (commands.filter((command) => command.id == id).length > 0) {

                newCommands = this._updateCommands(
                    commands,
                    id,
                    (command) => ({
                        ...command,
                        label: label,
                        value: value,
                        executedScript: script,
                        status: 'started'
                    })
                )
            } else {
                newCommands = [
                    ...commands,
                    {
                        id: id,
                        label: label,
                        value: value,
                        executedScript: script,
                        progressLines: [],
                        errorLines: [],
                        status: 'started',
                        duration: undefined
                    }
                ]
            }
            patchState(store, {
                commands: newCommands,
                currentTab: id,
                nextCommandParams: undefined,
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

        commandFailed(id: string, lines: string[], duration: number) {
            patchState(store, {
                commands: this._updateCommands(
                    store.commands(),
                    id,
                    (command) => ({
                        ...command,
                        status: 'failed',
                        errorLines: lines,
                        duration: duration,
                    })
                ),
            })
        },

        commandEnded(id: string, duration: number, statusCode: number) {
            patchState(store, {
                commands: this._updateCommands(
                    store.commands(),
                    id,
                    (command) => ({
                        ...command,
                        status: statusCode == 0 ? 'succeeded' : 'cancelled',
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

        replayCommand(id: string) {
            patchState(store, {
                commands: this._updateCommands(
                    store.commands(),
                    id,
                    (command) => ({
                        ...command,
                        status: 'started',
                        errorLines: [],
                        progressLines: [],
                        duration: undefined,
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
                invoke(HIDE_VIEW, {openTabs: false}).then()
            }

        },

        openConfirmKillProcDialog(id: string, closeTab: boolean) {
            patchState(store, {
                killProcDialogData: {commandId: id, closeTab: closeTab},
            })
        },

        closeConfirmKillProcDialog(killProc: boolean) {
            if (killProc) {
                const data = store.killProcDialogData()
                if (data) {
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