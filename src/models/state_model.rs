use super::{error_handler::*, model::SearchRecords};
use tui::widgets::ListState;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct Message {
    pub body: String,
}
impl Message {
    pub fn push_message(app: &mut App, message: Message) -> Result<(), CustomError> {
        if message.body != "" {
            app.messages.push(message.clone());
            let messages_need_trimming: bool = !app.messages.is_empty() && app.messages.len() > 20;
            if let true = messages_need_trimming {
                app.messages.remove(0);
            }
            Ok(())
        } else {
            return Err(CustomError {
                error_status_code: 400,
                error_message: "Message body is too short".to_string(),
            });
        }
    }
    pub fn configure_message(app: &mut App) -> Result<Message, CustomError> {
        let message_body: String = app.input.drain(..).collect();

        if message_body.len() > 0 {
            let message = Message { body: message_body };
            Ok(message)
        } else {
            Err(CustomError {
                error_status_code: 400,
                error_message: "Message body is empty".to_string(),
            })
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub enum InputMode {
    #[default]
    Editing,
}
#[derive(PartialEq, Debug)]
pub enum LoginInput {
    UserName,
    Password,
}
#[derive(Debug, Default)]
pub struct App {
    /// Current value of the input box
    pub input: String,
    pub messages: Vec<Message>,
    pub error_msg: UserErrorTypes,
    pub results: StatefulList<SearchRecords>,
    pub input_mode: InputMode,
    /// History of recorded messages
    pub login_hidden_input: String,
}

impl App {
    pub fn run_message_mode(&mut self) {
        self.input_mode = InputMode::Editing;
    }
    pub fn run_first_time_login(&mut self) {
        // self.user.save_local_preferences().unwrap();

        self.input_mode = InputMode::Editing;
        self.login_hidden_input.drain(..);
    }

    pub fn format_select(&self, index: usize) -> String {
        format!(
            "{}.{}",
            self.results.records[index].attributes.r#type, self.results.records[index].Id
        )
    }
}
#[derive(Debug, Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub records: Vec<T>,
}
impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            records: items,
        }
    }
    pub fn default_select(&mut self) {
        self.state.select(Some(0));
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.records.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.records.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
