use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::widgets::{Block, Borders};

pub fn render_layout(f: &mut Frame) -> RenderLayout {
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

pub struct RenderLayout {
    pub top: Rect,
    pub bottom: Rect,
    pub top_inner: Vec<Rect>,
    pub bottom_inner: Vec<Rect>,
    pub footer: Rect,
}