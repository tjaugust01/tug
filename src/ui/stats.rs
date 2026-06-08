use ratatui::Frame;
use ratatui::widgets::{Block, Borders};
use crate::app::App;

pub(crate) fn render(frame: &mut Frame, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Stats");
    frame.render_widget(block, frame.area())
}