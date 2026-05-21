use ratatui::layout::{Constraint, Direction, Flex};
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::{DefaultTerminal, Frame};
use ratatui::crossterm::event;
use ratatui::crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use crate::app::{ AppState};
use ratatui::layout::{Layout, Rect};
use ratatui::text::Line;
use crate::adapters::PackageManagerStatus;
use crate::app::state::Package;
use crate::config::package_managers::config::{APT, CARGO, SNAP};
use crate::config::package_managers::types::*;

pub fn render_main(state: &mut AppState) -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut state = state;

    ratatui::run(|term| app(term, &mut state))?;

    Ok(())
}

pub fn app(
    terminal: &mut DefaultTerminal,
    state: &mut AppState,
) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| {
            render(f, state);
        })?;
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }

                match key.code {
                    KeyCode::F(1) => { state.select_package_manager(APT)}
                    KeyCode::F(2) => { state.select_package_manager(SNAP)}
                    KeyCode::F(3) => { state.select_package_manager(CARGO)}
                    KeyCode::F(4) => {state.select_package(Package{ name: "".to_string() })}
                    _ => {}
                }
                if key.code == KeyCode::Esc {
                    return Ok(());
                }
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    return Ok(());
                }
            }
        }
    }

}
pub fn layout(f: &mut Frame) -> RenderLayout {
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(f.area());

    let top = main[0];
    let bottom = main[1];

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
    }
}

pub struct RenderLayout {
    pub top: Rect,
    pub bottom: Rect,
    pub top_inner: Vec<Rect>,
    pub bottom_inner: Vec<Rect>,
}

pub fn render(f: &mut Frame, state: &mut AppState){
let layout = layout(f);

    render_list_block(
        f,
        state,
        PackageManagerCategory::Distro,
        layout.top_inner[0]
    );

    render_list_block(
        f,
        state,
        PackageManagerCategory::User,
        layout.top_inner[1]
    );

    render_list_block(
        f,
        state,
        PackageManagerCategory::Language,
        layout.top_inner[2]
    );

    render_packages_block(
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
}

pub fn render_packages_block(
    f: &mut Frame,
    state: &mut AppState,
    area: Rect,
){
    let block = Block::default()
        .title(Line::from("Packages (F4)").left_aligned())
        .title(Line::from("(grouped)").right_aligned())

        .borders(Borders::ALL);

    f.render_widget(&block, area);

    let inner = block.inner(area);

    let items: Vec<ListItem> = state
        .packaged_grouped_by_manager
        .get("APT")
        .into_iter()
        .flat_map(|v| v.iter())
        .map(|p| ListItem::new(p.0.name.clone()))
        .collect();

    let list = List::new(items);

    f.render_widget(list, inner);
}

pub fn render_list_block(
    f: &mut Frame,
    state: &mut AppState,
    category: PackageManagerCategory,
    area: Rect,
) {
    let block = Block::default()
        .title(category.to_string())
        .borders(Borders::ALL);

    f.render_widget(&block, area);

    let inner = block.inner(area);

    let items: Vec<ListItem> = state
        .detection_package_managers
        .iter()
        .filter(|x| x.status == PackageManagerStatus::Installed)
        .filter(|x| x.manager.category == category)
        .map(|x| ListItem::new(x.manager.name.to_string()))
        .collect();

    let list = List::new(items);

    f.render_widget(list, inner);
}