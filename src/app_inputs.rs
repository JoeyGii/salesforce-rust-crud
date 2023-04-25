use crate::models::state_model::*;
use crate::ui::ui_render_handler;
use crate::{
    authorize, configure_updates, describe_fields, describe_filtered_fields, export_fields,
    get_ids_cli,
};
use crossterm::event::{self, Event, KeyCode};
use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};
use tui::{backend::Backend, Terminal};
const MILLIS_SLEEP_DURATION: Duration = Duration::from_millis(10);

pub async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    inputted_app: Arc<Mutex<App>>,
) -> Result<(), Box<dyn Error>> {
    loop {
        let mut app = inputted_app.lock().unwrap();
        terminal.draw(|f| ui_render_handler::ui(f, &mut app))?;

        if event::poll(MILLIS_SLEEP_DURATION).unwrap() {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => {
                        app.results.next();
                        app.input = app.format_select(app.results.state.selected().unwrap());
                    }
                    KeyCode::Up => {
                        app.results.previous();
                        app.input = app.format_select(app.results.state.selected().unwrap());
                    }
                    KeyCode::Char('`') => {
                        let new_msg = Message::configure_message(&mut app).unwrap();
                        let token: String = authorize().await?.access_token;
                        let inputted_string: Vec<&str> = new_msg.body.split(".").collect();
                        let result = describe_filtered_fields(
                            &token,
                            inputted_string[0],
                            inputted_string[1],
                            inputted_string[2],
                        )
                        .await;
                        match result {
                            Ok(r) => app.messages = r,
                            Err(e) => {
                                let m: Message = Message {
                                    body: e.to_string(),
                                };
                                app.messages = vec![m]
                            }
                        }
                    }
                    KeyCode::Enter => {
                        let new_msg = Message::configure_message(&mut app).unwrap();

                        let token: String = authorize().await?.access_token;
                        let get: Vec<&str> = new_msg.body.split(".").collect();

                        if get.len() > 2 {
                            let fields: Vec<String> =
                                get[2].split(",").map(|s| s.to_string()).collect();
                            let a: crate::models::arg_model::Args =
                                crate::models::arg_model::Args {
                                    sobj: get[0].to_string(),
                                    id: get[1].to_string(),
                                    fields: fields,
                                };
                            configure_updates(a, &token, &mut app).await?
                        }
                        app.results =
                            StatefulList::with_items(get_ids_cli(&token, &get[0], &get[1]).await?);
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Tab => {
                        let new_msg = Message::configure_message(&mut app).unwrap();
                        let token: String = authorize().await?.access_token;
                        let inputted_string: Vec<&str> = new_msg.body.split(".").collect();
                        let result =
                            describe_fields(&token, inputted_string[0], inputted_string[1]).await;
                        match result {
                            Ok(r) => app.messages = r,
                            Err(e) => {
                                let m: Message = Message {
                                    body: e.to_string(),
                                };
                                app.messages = vec![m]
                            }
                        }
                        let fields = app.messages.iter().map(|b| b.body.to_string()).collect();
                        if let Err(e) = export_fields(fields) {
                            panic!("Could not export CSV {}", e);
                        }
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        } else {
            drop(app);
            std::thread::sleep(MILLIS_SLEEP_DURATION);
        }
    }
}
