use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::widgets::{Block, Borders};
use crate::app::AppState;
use crate::config::package_manager::PackageManagerCategory;
use crate::presentation::render::block;

pub fn layout(f: &mut Frame) -> RenderLayout {
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(78),
            Constraint::Percentage(2),
        ])
        .split(f.area());

    let top = main[0];
    let bottom = main[1];
    let footer = main[2];

    let top_block = Block::default().borders(Borders::ALL).title("Package managers");
    f.render_widget(top_block.clone(), top);

    let inner_top = top_block.inner(top);

    let top_inner = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(inner_top)
        .to_vec();

    let bottom_inner = Layout::default()
        .flex(Flex::Start)
        .spacing(1)
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(bottom)
        .to_vec();

    RenderLayout {
        top,
        bottom,
        top_inner,
        bottom_inner,
        footer,
    }
}


pub fn render_layout(f: &mut Frame, state: &mut AppState){
    let layout = layout(f);

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

pub struct RenderLayout {
    pub top: Rect,
    pub bottom: Rect,
    pub top_inner: Vec<Rect>,
    pub bottom_inner: Vec<Rect>,
    pub footer: Rect,
}