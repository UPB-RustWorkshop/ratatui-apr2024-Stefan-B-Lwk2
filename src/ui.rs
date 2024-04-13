use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph};
use ratatui::Frame;
use ratatui::style::{Color, Modifier, Style};
use crate::app::App;



/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.

    let vertical = Layout::default().direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(90),
      
        Constraint::Percentage(10),
    ]).split(frame.size());
        //FACE MAP, Cu ce primeste din App
    let items: Vec<ListItem> = app.cities
    .iter().map(|city| {
        ListItem::new(Line::raw(city))
    })
    .collect();

    let list = List::new(items)
    .block(Block::default().title("List").borders(Borders::ALL))
    .style(Style::default().fg(Color::Red))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::TopToBottom);



    frame.render_stateful_widget(list, vertical[0],&mut app.state);
    frame.render_widget(Paragraph::new(app.info.to_string()), vertical[1] );
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
 
    
    // TODO: Split the layout
    // let [area1, area2, area3 ...] = 

    // TODO: get the list of cities
    // let cities: Vec<ListItem> =
    // let list_component = 

    // TODO: render the list of cities
    // frame.render_widget(list_component, area);


    // TODO: Create the weather info component
    // let weather_info = 

    // TODO: Render the weather info component
    // frame.render_widget(weather_info, area);


}
