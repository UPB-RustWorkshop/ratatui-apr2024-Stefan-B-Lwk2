use std::error;

use ratatui::widgets::ListState;

use crate::connection::{get_cities, get_temperature};

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub cities: Vec<String>,
    pub running: bool,
    pub state: ListState,
    pub info: f32,
}


impl App {
    /// Constructs a new instance of [`App`].
    
    pub async fn new() -> Self {
        Self {
            running: true,
            //cities:vec![String::from("Bucharest"),String::from("New York"),String::from("London")],
            cities: get_cities().await,
            state: ListState::default(),
            info: 0.0,
        }
    }

   pub async fn downInc(&mut self){
        if let Some(index) = self.state.selected() {
            match get_temperature( self.cities[index].clone()).await{
                Ok(x) => {self.info = x.conditions.temp},
                Err(_) => {} 
            }

            if index >= (self.cities.len()- 1) {
                self.state.select(Some(0));
            }
            
            else { self.state.select(Some(index+1));
            }
            

        }
        else {
            self.state.select(Some(0));
        }
    }

    pub async fn upInc(&mut self){
        
        if let Some(index) = self.state.selected() {

            match get_temperature( self.cities[index].clone()).await{
                Ok(x) => {self.info = x.conditions.temp},
                Err(_) => {} 
            }

            if index == 0 {
                self.state.select(Some(self.cities.len()-1));
            }
        
            else {
                self.state.select(Some(index-1));
            }
            
        }
        else {
            self.state.select(Some(self.cities.len()));
        }
    }
}
