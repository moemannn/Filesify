use ratatui::Frame;
use ratatui::prelude::Line;
use ratatui::widgets::{Block, Borders};
use crate::app::AppState;
use crate::config::package_manager::PackageManagerCategory;
use super::*;

pub fn render_layout(f: &mut Frame, state: &mut AppState){
    let layout = layout::render_layout(f);

    block::render_list(
        f,
        state,
        PackageManagerCategory::Distro,
        layout.top_inner[0]
    );

    block::render_list(
        f,
        state,
        PackageManagerCategory::User,
        layout.top_inner[1]
    );

    block::render_list(
        f,
        state,
        PackageManagerCategory::Language,
        layout.top_inner[2]
    );

    block::render_packages(
        f,
        state,
        layout.bottom_inner[0]
    );

    f.render_widget(
        Block::default()
            .title(Line::from("Info").left_aligned())
            .title(Line::from("Middle Title").centered())
            .title(Line::from("Right Title").right_aligned())
            .borders(Borders::ALL),
        layout.bottom_inner[1],
    );
    block::render_footer(f, layout.footer);
}