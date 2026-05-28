use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::process;
use crate::app::AppState;
use crate::config::package_manager::config::{APT, CARGO, SNAP};

pub fn key_loop(state: &mut AppState) -> std::io::Result<()> {
    enable_raw_mode()?;

    loop {
        if let Event::Key(event) = read()? {
            match event.code {


                KeyCode::F(1) => { state.select_package_manager(APT)}
                KeyCode::F(2) => { state.select_package_manager(SNAP)}
                KeyCode::F(3) => { state.select_package_manager(CARGO)}

                // KeyCode::F(4) => { state.select_package_count(CARGO)}

                KeyCode::Char('q') => {
                    return Ok(());
                }

                other => {}
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}