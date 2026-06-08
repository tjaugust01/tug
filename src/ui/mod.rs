use ratatui::Frame;
use crate::app::{App, Tab};

mod stats; 
mod compose;
mod images;
mod containers;

pub fn render(frame: &mut Frame, app: &App) {
    match app.tab {
        Tab::Containers => containers::render(frame, app),
        Tab::Images     => images::render(frame, app),
        Tab::Compose    => compose::render(frame, app),
    }
}