mod board;
mod card;
mod column;
pub mod theme;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::{App, AppMode};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Fill background
    frame.render_widget(
        Block::default().style(Style::default().bg(theme::BG_BASE)),
        area,
    );

    // Outer layout: header + board + status bar
    let outer = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(area);

    render_header(frame, outer[0]);
    board::render_board(frame, app, outer[1]);
    render_status_bar(frame, app, outer[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let title = Line::from(vec![
        Span::styled(
            "slate",
            Style::default()
                .fg(theme::TITLE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " — personal task planner",
            Style::default().fg(theme::TITLE_DIM),
        ),
    ]);

    frame.render_widget(
        Paragraph::new(title)
            .alignment(Alignment::Center)
            .style(Style::default().bg(theme::BG_BASE)),
        area,
    );
}


fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let line = match &app.mode {
        AppMode::Normal => Line::from(vec![
            Span::styled(" h/l", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(" columns  ", Style::default().fg(theme::FG_DIM)),
            Span::styled("j/k", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(" scroll  ", Style::default().fg(theme::FG_DIM)),
            Span::styled("n", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(" new  ", Style::default().fg(theme::FG_DIM)),
            Span::styled("H/L", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(" move  ", Style::default().fg(theme::FG_DIM)),
            Span::styled("d", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(" delete  ", Style::default().fg(theme::FG_DIM)),
            Span::styled("q", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(" quit", Style::default().fg(theme::FG_DIM)),
        ]),
        AppMode::Insert => Line::from(vec![
            Span::styled(" Type issue title, ", Style::default().fg(theme::FG_DIM)),
            Span::styled("Enter", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(" save  ", Style::default().fg(theme::FG_DIM)),
            Span::styled("Esc", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(" cancel", Style::default().fg(theme::FG_DIM)),
        ]),
        AppMode::Confirm => Line::from(vec![
            Span::styled(" Delete issue? ", Style::default().fg(theme::FG_CONFIRM)),
            Span::styled("y", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(" yes  ", Style::default().fg(theme::FG_DIM)),
            Span::styled("n/Esc", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(" cancel", Style::default().fg(theme::FG_DIM)),
        ]),
    };

    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(theme::BG_BASE)),
        area,
    );
}
