use color_eyre::eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::{App, AppMode};

pub fn handle_event(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }

            // Ctrl-C always quits
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                app.should_quit = true;
                return Ok(());
            }

            match app.mode {
                AppMode::Normal => handle_normal(app, key)?,
                AppMode::Insert => handle_insert(app, key)?,
                AppMode::Confirm => handle_confirm(app, key)?,
            }
        }
    }
    Ok(())
}

fn handle_normal(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,

        // Column navigation
        KeyCode::Char('h') | KeyCode::Left => app.move_column_left(),
        KeyCode::Char('l') | KeyCode::Right => app.move_column_right(),

        // Card navigation
        KeyCode::Char('j') | KeyCode::Down => {
            app.active_col_mut().select_next();
            adjust_scroll(app);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.active_col_mut().select_prev();
            adjust_scroll(app);
        }
        KeyCode::Char('g') => {
            app.active_col_mut().select_first();
            adjust_scroll(app);
        }
        KeyCode::Char('G') => {
            app.active_col_mut().select_last();
            adjust_scroll(app);
        }

        // Create issue
        KeyCode::Char('n') | KeyCode::Char('a') => {
            app.input.clear();
            app.mode = AppMode::Insert;
        }

        // Delete issue
        KeyCode::Char('d') => {
            if app.active_col().selected_issue().is_some() {
                app.mode = AppMode::Confirm;
            }
        }

        // Move forward (Todo -> InProgress -> Done)
        KeyCode::Char('m') => {
            app.move_selected_forward()?;
        }

        // Move backward (Done -> InProgress -> Todo)
        KeyCode::Char('M') => {
            app.move_selected_backward()?;
        }

        _ => {}
    }
    Ok(())
}

fn handle_insert(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.input.clear();
            app.mode = AppMode::Normal;
        }
        KeyCode::Enter => {
            app.create_issue()?;
            app.mode = AppMode::Normal;
            adjust_scroll(app);
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        _ => {}
    }
    Ok(())
}

fn handle_confirm(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => {
            app.delete_selected()?;
            app.mode = AppMode::Normal;
            adjust_scroll(app);
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            app.mode = AppMode::Normal;
        }
        _ => {}
    }
    Ok(())
}

/// Adjust scroll_offset so the selected card is visible.
/// Assumes a rough terminal height and 5-line cards.
fn adjust_scroll(app: &mut App) {
    let col = &mut app.columns[app.active_column];
    if col.issues.is_empty() {
        col.scroll_offset = 0;
        return;
    }

    // Ensure selected is within bounds
    if col.selected >= col.issues.len() {
        col.selected = col.issues.len() - 1;
    }

    // If selected is above the scroll window, scroll up
    if col.selected < col.scroll_offset {
        col.scroll_offset = col.selected;
    }

    // Estimate how many cards fit in the visible area.
    // We don't have the exact area height here, so we use a conservative estimate.
    // Each card is 5 lines, header is 2 lines, column border is 2 lines.
    // This is a rough heuristic; the rendering will handle the actual clipping.
    let visible_cards = 6; // conservative estimate for a typical terminal
    if col.selected >= col.scroll_offset + visible_cards {
        col.scroll_offset = col.selected - visible_cards + 1;
    }
}
