use dotenv::dotenv;
use sf_updates::models::state_model::App;

mod models {

    pub mod error_handler;
}

use sf_updates::app_inputs;

use std::{
    io,
    sync::{Arc, Mutex},
};
use tui::{backend::CrosstermBackend, Terminal};
extern crate preferences;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = Arc::new(Mutex::new(App::default()));
    let res = app_inputs::run_app(&mut terminal, app).await;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
