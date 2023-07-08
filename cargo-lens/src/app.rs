use std::marker::PhantomData;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::review_req_checklist::ReviewReqChecklist;

/// Central hub for data/widget reference.
pub struct App<B> {
    /// raw-data on checklist-items
    pub list: ReviewReqChecklist,
    _phantom: PhantomData<B>,
}
impl<B: Backend> App<B> {
    pub fn new(list: ReviewReqChecklist) -> Self {
        Self {
            list,
            _phantom: PhantomData,
        }
    }

    pub fn render(&self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Min(0)].as_ref())
            .split(f.size());
        let items = self.lines();

        let block = Block::default().title("Checklist").borders(Borders::ALL);
        let checklist = List::new(items).block(block);
        f.render_widget(checklist, chunks[0]);
        let block = Block::default().title("Info").borders(Borders::ALL);
        let info =
            Paragraph::new::<&str>(self.list.items.get(self.list.index).unwrap().info.as_ref())
                .block(block);
        f.render_widget(info, chunks[1]);
    }
    pub fn lines(&self) -> Vec<ListItem> {
        let tick = "✓";
        let cross = "×";
        let span = |fill| -> Vec<Span> {
            ["[", fill, "] - "]
                .into_iter()
                .map(|st: &str| Span::raw(st))
                .collect()
        };

        let lines: Vec<ListItem> = self
            .list
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let mut spans = if item.toggled {
                    span(tick)
                } else {
                    span(cross)
                };
                spans.push(Span::raw(&item.name));
                let res = ListItem::new(Line::from(spans));
                if i == self.list.index {
                    res.style(Style::default().add_modifier(Modifier::BOLD))
                } else {
                    res
                }
            })
            .collect();
        lines
    }
}
