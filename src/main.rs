mod app;
mod db;
mod event;
mod model;
mod ui;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();

    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    let mut app = app::App::new()?;
    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        event::handle_event(&mut app)?;
        if app.should_quit {
            break;
        }
    }
    Ok(())
}
