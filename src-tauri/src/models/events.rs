use crate::models::errors::AppErrors;
use crate::models::errors::AppErrors::EmitEventError;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

pub(crate) trait Event {
    fn name(&self) -> &'static str;
}
pub mod commands {
    use crate::models::events::Event;
    use uuid::Uuid;


    #[derive(Clone, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CommandRequested<'a> {
        pub command_id: Uuid,
        pub command_label: &'a str,
        pub command_value: &'a str,
        pub command_to_execute: &'a str,
        pub command_params: Option<Vec<String>>
    }

    impl Event for  CommandRequested<'_> {
        fn name(&self) -> &'static str {
            "COMMAND_REQUESTED"
        }
    }

    #[derive(Clone, serde::Serialize)]
    #[serde(rename_all = "camelCase", tag ="event", content = "data")]
    pub enum CommandExecutionEvent {
        #[serde(rename_all = "camelCase")]
        CommandStarted,
        #[serde(rename_all = "camelCase")]
        CommandProgress {
            progress_line: String
        },
        #[serde(rename_all = "camelCase")]
        CommandEnded {
            duration: u128,
            status_code: i32
        },
        #[serde(rename_all = "camelCase")]
        CommandFailed {
            errors_lines: Vec<String>,
            duration: u128,
            status_code: i32
        }
    }
   

}

pub(crate) fn  emit_event<T : Event + Serialize + Clone>(app: &AppHandle, event: T) -> Result<(), AppErrors>  {
    app.emit(event.name(), event).map_err(|e| EmitEventError(e.to_string()))
}
