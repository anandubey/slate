use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::{AppMode, ColumnState};
use crate::model::Status;
use super::card::render_card;
use super::theme;

pub fn render_column(
    frame: &mut Frame,
    col: &ColumnState,
    area: Rect,
    is_active: bool,
    mode: &AppMode,
    input: &str,
) {
    let border_color = if is_active {
        theme::FG_BORDER_ACTIVE
    } else {
        theme::FG_BORDER
    };

    let column_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme::BG_COLUMN));

    let column_inner = column_block.inner(area);
    frame.render_widget(column_block, area);

    if column_inner.width == 0 || column_inner.height == 0 {
        return;
    }

    // Split into header + card area (+ optional input area)
    let show_input = is_active && *mode == AppMode::Insert;
    let constraints = if show_input {
        vec![
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]
    } else {
        vec![Constraint::Length(2), Constraint::Min(0)]
    };
    let chunks = Layout::vertical(constraints).split(column_inner);

    render_header(frame, col, chunks[0], is_active);
    render_card_list(frame, col, chunks[1], is_active);

    if show_input && chunks.len() > 2 {
        render_input(frame, input, chunks[2]);
    }
}

fn render_header(frame: &mut Frame, col: &ColumnState, area: Rect, is_active: bool) {
    let status_color = match col.status {
        Status::Todo => theme::STATUS_TODO,
        Status::InProgress => theme::STATUS_IN_PROGRESS,
        Status::Done => theme::STATUS_DONE,
    };

    let visible = col.issues.len();
    let total = col.total_count;

    let name_style = if is_active {
        Style::default()
            .fg(theme::FG_PRIMARY)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::FG_SECONDARY)
    };

    let header_line = Line::from(vec![
        Span::raw(" "),
        Span::styled(col.status.icon(), Style::default().fg(status_color)),
        Span::raw(" "),
        Span::styled(col.status.display_name(), name_style),
        Span::raw("  "),
        Span::styled(
            format!("{visible}/{total}"),
            Style::default().fg(theme::FG_DIM),
        ),
    ]);

    frame.render_widget(
        Paragraph::new(header_line).style(Style::default().bg(theme::BG_COLUMN)),
        area,
    );
}

fn render_card_list(frame: &mut Frame, col: &ColumnState, area: Rect, is_active: bool) {
    if col.issues.is_empty() {
        render_empty_state(frame, col.status, area, is_active);
        return;
    }

    let mut y = area.y;
    let card_width = area.width;

    for (i, issue) in col.issues.iter().enumerate().skip(col.scroll_offset) {
        let h = issue.card_height();
        if y + h > area.y + area.height {
            break;
        }

        let card_area = Rect::new(area.x, y, card_width, h);
        let is_selected = is_active && i == col.selected;
        render_card(frame, issue, card_area, is_selected);
        y += h;
    }

    // Scroll indicators
    if col.scroll_offset > 0 {
        let indicator = Line::from(vec![
            Span::styled(" ▲ ", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(
                format!("{} more", col.scroll_offset),
                Style::default().fg(theme::FG_DIM),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(indicator).style(Style::default().bg(theme::BG_COLUMN)),
            Rect::new(area.x, area.y, area.width, 1),
        );
    }

    let mut used_height = 0u16;
    let mut visible_count = 0usize;
    for issue in col.issues.iter().skip(col.scroll_offset) {
        let h = issue.card_height();
        if used_height + h > area.height {
            break;
        }
        used_height += h;
        visible_count += 1;
    }
    let last_visible = col.scroll_offset + visible_count;
    if last_visible < col.issues.len() {
        let below = col.issues.len() - last_visible;
        let indicator = Line::from(vec![
            Span::styled(" ▼ ", Style::default().fg(theme::FG_ACCENT)),
            Span::styled(
                format!("{below} more"),
                Style::default().fg(theme::FG_DIM),
            ),
        ]);
        if area.height > 0 {
            frame.render_widget(
                Paragraph::new(indicator).style(Style::default().bg(theme::BG_COLUMN)),
                Rect::new(area.x, area.y + area.height - 1, area.width, 1),
            );
        }
    }
}

fn render_empty_state(frame: &mut Frame, status: Status, area: Rect, is_active: bool) {
    let (icon, message, hint_color) = match status {
        Status::Todo => (
            "    ○",
            "Nothing on your slate",
            theme::STATUS_TODO,
        ),
        Status::InProgress => (
            "    ◐",
            "All clear for now",
            theme::STATUS_IN_PROGRESS,
        ),
        Status::Done => (
            "    ●",
            "Finish something!",
            theme::STATUS_DONE,
        ),
    };

    let hint = if is_active {
        "press 'n' to add"
    } else {
        ""
    };

    let text = Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(
            icon,
            Style::default().fg(hint_color),
        )),
        Line::from(""),
        Line::from(Span::styled(
            message,
            Style::default().fg(theme::FG_DIM),
        )).alignment(Alignment::Center),
        Line::from(Span::styled(
            hint,
            Style::default()
                .fg(theme::FG_ACCENT)
                .add_modifier(Modifier::ITALIC),
        )).alignment(Alignment::Center),
    ]);

    frame.render_widget(
        Paragraph::new(text).style(Style::default().bg(theme::BG_COLUMN)),
        area,
    );
}

fn render_input(frame: &mut Frame, input: &str, area: Rect) {
    let input_block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(theme::FG_BORDER_ACTIVE))
        .style(Style::default().bg(theme::BG_INPUT));

    let inner = input_block.inner(area);
    frame.render_widget(input_block, area);

    let line = Line::from(vec![
        Span::styled(" + ", Style::default().fg(theme::FG_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(input, Style::default().fg(theme::FG_PRIMARY)),
        Span::styled("▎", Style::default().fg(theme::FG_ACCENT)),
    ]);

    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(theme::BG_INPUT)),
        inner,
    );
}
