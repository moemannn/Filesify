use ratatui::DefaultTerminal;

use crate::app::AppState;
use crate::presentation::render::ui;
use super::input;

pub fn app(state: &mut AppState) -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut state = state;

    // ratatui::run(|term| main_loop(term, &mut state))?;

    Ok(())
}

fn main_loop(
    terminal: &mut DefaultTerminal,
    state: &mut AppState,
) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            ui::render_layout(f, state);
        })?;

        input::handler::key_loop(state)?;
        return Ok(())
    }

}


