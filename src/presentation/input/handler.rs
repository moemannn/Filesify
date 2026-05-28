use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::process;
use crate::app::AppState;
use crate::config::package_manager::config::{APT, CARGO, SNAP, *};
use crate::config::package_manager::PackageManagerCategory;
use crate::config::package_manager::PackageManagerCategory::{Distro, Language, User};

pub fn key_loop(state: &mut AppState) -> std::io::Result<()> {
    enable_raw_mode()?;

    loop {
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::F(1) => {
                    change_category_manager(state, Distro)
                }

                KeyCode::F(2) => {
                    change_category_manager(state, User)
                }

                KeyCode::F(3) => {
                    change_category_manager(state, Language)
                }

                KeyCode::Char('q') => {
                    break;
                }

                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

fn change_category_manager(
    state: &mut AppState,
    manager: PackageManagerCategory
){
    state.set_category(manager)
}