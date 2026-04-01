use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::model::{IssueSummary, Priority, Status};
use super::theme;

pub fn render_card(frame: &mut Frame, issue: &IssueSummary, area: Rect, is_selected: bool) {
    let bg = if is_selected {
        theme::BG_CARD_SELECTED
    } else {
        theme::BG_CARD
    };

    let border_color = if is_selected {
        theme::FG_BORDER_ACTIVE
    } else {
        theme::FG_BORDER
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .padding(Padding::horizontal(1))
        .style(Style::default().bg(bg));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width == 0 || inner.height == 0 {
        return;
    }

    let status_color = match issue.status {
        Status::Todo => theme::CARD_ACCENT_TODO,
        Status::InProgress => theme::CARD_ACCENT_IN_PROGRESS,
        Status::Done => theme::CARD_ACCENT_DONE,
    };

    let status_icon = match issue.status {
        Status::Todo => "○",
        Status::InProgress => "◐",
        Status::Done => "✓",
    };

    // Line 1: Status dot + Issue ID (left) + Priority icon (right)
    let id_part = format!("{} {}", status_icon, issue.issue_id);
    let id_span = Span::styled(&id_part, Style::default().fg(status_color));
    let priority_span = Span::styled(
        issue.priority.icon(),
        Style::default().fg(priority_color(issue.priority)),
    );

    let id_display_len = id_part.chars().count();
    let pri_display_len = issue.priority.icon().chars().count();
    let padding = (inner.width as usize).saturating_sub(id_display_len + pri_display_len);

    let line1 = Line::from(vec![
        id_span,
        Span::raw(" ".repeat(padding)),
        priority_span,
    ]);

    // Line 2: Title (bold, truncated)
    let max_title_width = inner.width as usize;
    let title_display = truncate(&issue.title, max_title_width);
    let title_style = if issue.status == Status::Done {
        Style::default()
            .fg(theme::FG_SECONDARY)
            .add_modifier(Modifier::CROSSED_OUT)
    } else {
        Style::default()
            .fg(theme::FG_PRIMARY)
            .add_modifier(Modifier::BOLD)
    };
    let line2 = Line::from(Span::styled(title_display, title_style));

    // Line 3: Created date
    let created_str = issue.created_at.format("Created %b %Y").to_string();
    let line3 = Line::from(Span::styled(created_str, Style::default().fg(theme::FG_DIM)));

    let text = Text::from(vec![line1, line2, line3]);
    frame.render_widget(Paragraph::new(text), inner);

    // Draw a colored left accent bar (overwrite the left border chars at rows 1-3)
    if is_selected && area.height >= 3 {
        for dy in 1..area.height.saturating_sub(1) {
            frame.render_widget(
                Paragraph::new(Span::styled(
                    "┃",
                    Style::default().fg(status_color).bg(bg),
                )),
                Rect::new(area.x, area.y + dy, 1, 1),
            );
        }
    }
}

fn priority_color(priority: Priority) -> ratatui::style::Color {
    match priority {
        Priority::None => theme::PRIORITY_NONE,
        Priority::Low => theme::PRIORITY_LOW,
        Priority::Medium => theme::PRIORITY_MEDIUM,
        Priority::High => theme::PRIORITY_HIGH,
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_len {
        s.to_string()
    } else if max_len > 3 {
        let truncated: String = s.chars().take(max_len - 3).collect();
        format!("{truncated}...")
    } else {
        s.chars().take(max_len).collect()
    }
}
