import {Component, computed, effect, ElementRef, inject, OnInit, viewChild} from '@angular/core';
import {AppStore} from "../../app.store";
import {NgClass} from "@angular/common";
import {AppService} from "../../app.service";

@Component({
    selector: 'app-command-view',
    imports: [
        NgClass
    ],
    templateUrl: './command-view.component.html',
    styleUrl: './command-view.component.css'
})
export class CommandViewComponent implements OnInit {

    appStore = inject(AppStore);

    appService = inject(AppService)

    command = this.appStore.currentCommand

    resultView = viewChild<ElementRef<HTMLDivElement>>('command_result');

    lines = computed(() => this.command()?.progressLines ?? [])

    errorLines = computed(() => this.command()?.errorLines ?? [])

    duration = computed(() => {
        let duration = this.command()?.duration
        if (duration) {
            return duration/1000
        }
        return undefined
    })

    isWaiting = computed(() => this.command()?.status === 'waiting_for_params')

    isRunning = computed(() => this.command()?.status === 'in_progress' || this.command()?.status === 'started')

    commandClass = computed(() => ({
        'in-progress': this.command()?.status === 'in_progress',
        'cancelled': this.command()?.status === 'cancelled',
        'errors': this.errorLines()?.length > 0,
    }))

    _e = effect(() => {

        this.lines()

        let elmt = this.resultView()?.nativeElement;

        if (elmt) {
            elmt.scroll({
                behavior: 'smooth',
                top: elmt.scrollHeight + 40
            })
        }
    })

    ngOnInit(): void {
        document.addEventListener('keydown', (e) => {
            const cmd = this.command()
            if (cmd && this.isRunning() && e.ctrlKey && e.key === 'c') {
                this.appStore.openConfirmKillProcDialog(cmd.id, false)
            }

        })
    }

    replay() {
        let command = this.command();
        if (command) {
            this.appService.executeCommand(command.id, command.executedScript)
        }
    }
}
