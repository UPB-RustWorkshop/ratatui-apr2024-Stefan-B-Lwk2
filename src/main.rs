use crossterm::event::{KeyCode, KeyEvent};
use ratatui_templates::app::{App, AppResult};
use ratatui_templates::connection::get_temperature;
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
     let mut app = App::new().await;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    // TODO:  the terminal user interface
    // let mut tui =
    
    // TODO: init the terminal

    //FOLOSIRE API DE TEST 
  //  get_temperature(String::from("Bucharest")).await;

    // Start the main loop.
    let mut tui = Tui::new(terminal,EventHandler::new(10));
    tui.init()?;
     while app.running {
        // TODO: Render the user interface.
        tui.draw(&mut app);


        // TODO: Handle events.
        
        match tui.events.next().await?{
            Event::Key(key) => {match key.code {
                KeyCode::Esc => {tui.exit(); break;}
                KeyCode::Down => {app.downInc().await}
                KeyCode::Up => {app.upInc().await}
               // KeyCode::Enter => {}
                _ => {}}}
            
            Event::Mouse(_)=> {tui.exit(); println!("Nu a fost Esc, a fost mouse");break;}

            _ => {}

        }
     }

    // TODO: Reset the terminal if the app has been terminated

    Ok(())
}
