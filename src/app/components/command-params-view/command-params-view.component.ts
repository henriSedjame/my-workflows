import {Component, computed, inject, signal} from '@angular/core';
import {AppStore} from "../../app.store";
import {AppService} from "../../app.service";
import {NgClass} from "@angular/common";

@Component({
    selector: 'app-command-params-view',
    imports: [
        NgClass
    ],
    templateUrl: './command-params-view.component.html',
    styleUrl: './command-params-view.component.css'
})
export class CommandParamsViewComponent {
    appStore = inject(AppStore)
    appService = inject(AppService)
    command = this.appStore.currentCommand
    params = this.appStore.nextCommandParams

    filledParams = signal<string[]>([])

    ready = computed(() => {
        return this.params()?.every(param => param.value != undefined && param.value != '') ?? false
    })

    btnClass = computed(() => ({
        'ready': this.ready()
    }))

    script = computed(() => {
        let script = this.command()?.executedScript
        this.params()?.forEach(param => {
            const name = '${param.' + param.name + '}'
            const value = param.value
            if (value != undefined && value != '') {
                script = script?.replace(name, value)
            }

        })
        return script
    })

    command_value = computed(() => {
        let script = this.command()?.value
        this.params()?.forEach(param => {
            const name = '${param.' + param.name + '}'
            const value = param.value
            if (value != undefined && value != '') {
                script = script?.replace(name, value)
            }
        })
        return script
    })

    placeholder(name: string) {
        return `Enter the parameter ${name} value`
    }
    updateParam($event: Event, name: string) {
        let value = ($event.target as HTMLInputElement)?.value
        this.filledParams.update((p) => {
            if (p.includes(name)) {
                return p
            }
            return [...p, name]
        })

        this.appStore.setNextCommandParamValue(name, value)
    }

    onInput(name:string) {
        this.filledParams.update((p) => p.filter(p => p != name))
    }

    sendCommand() {
        if (this.ready()) {
            const cmd = this.command()!
            this.appService.executeCommand(
                cmd.id,
                this.script()!,
                {
                    value: this.command_value()!,
                    label: cmd.label,
                    parameters: undefined
                }
            )
        }
    }

    cancel() {
        this.appStore.cancelNextCommandParams()
    }
}
