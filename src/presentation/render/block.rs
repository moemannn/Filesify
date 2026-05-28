use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Line, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Padding, Paragraph};
use crate::adapters::{PackageManagerResult, PackageManagerStatus};
use crate::app::AppState;
use crate::config::package_manager::{PackageManager, PackageManagerCategory};

pub fn render_footer(
    f: &mut Frame,
    layout: Rect
) {
    f.render_widget(
        Paragraph::new(
            "[Q] Quit app │ \
            [F1] Distro │ \
            [F2] User │ \
            [F3] Language │ \
            [F4] Package │ \
            [F5] Info manager"
        )
            .alignment(Alignment::Left)
            .style(Style::new().fg(Color::Black).bg(Color::White)),
        layout,
    );
}

pub fn render_packages(
    f: &mut Frame,
    state: &mut AppState,
    area: Rect,
){
    let block = Block::default()
        .title(Line::from("Packages").left_aligned())
        .title(Line::from("(grouped)").right_aligned())
        .padding(Padding::new(1,1,1,1))

        .borders(Borders::ALL);

    f.render_widget(&block, area);

    let inner = block.inner(area);

    let items: Vec<ListItem> = state
        .detected_package_grouped
        .get("APT")
        .into_iter()
        .flat_map(|v| v.iter())
        .map(|p| ListItem::new(p.0.name.clone()))
        .collect();

    let list = List::new(items);

    f.render_widget(list, inner);
}

pub fn render_list(
    f: &mut Frame,
    state: &mut AppState,
    category: PackageManagerCategory,
    area: Rect,
) {
    let block = Block::default()
        .title(category.to_string())
        .borders(Borders::ALL)
        .padding(Padding::new(1,1,0,0))
        ;

    f.render_widget(&block, area);

    let inner = block.inner(area);

    let items: Vec<ListItem> = state
        .detected_package_managers
        .iter()
        .filter(|x| x.status == PackageManagerStatus::Installed)
        .filter(|x| x.manager.category == category)
        .map(|x| select_item(state, x.manager))
        .collect();

    let list = List::new(items);

    dbg!(&list);

    f.render_widget(list, inner);
}

fn select_item(state: &mut AppState, list_item: &PackageManager) -> ListItem<'static> {
    let mut item = ListItem::new(list_item.name.clone());

    if state.selected_package_manager.unwrap().name == list_item.name {
        item = item.style(Style::new().fg(Color::Black).bg(Color::White));
    }

    item
}