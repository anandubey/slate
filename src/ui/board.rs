use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::app::App;
use super::column::render_column;

pub fn render_board(frame: &mut Frame, app: &App, area: Rect) {
    let columns = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .spacing(1)
    .split(area);

    for (i, col_area) in columns.iter().enumerate() {
        let is_active = i == app.active_column;
        render_column(
            frame,
            &app.columns[i],
            *col_area,
            is_active,
            &app.mode,
            &app.input,
        );
    }
}
