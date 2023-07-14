use std::marker::PhantomData;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::components::{checklist::ReviewReqChecklist, diagnostic::DiagParagraph};

/// Central hub for data/widget reference.
pub struct App<B> {
    /// raw-data on checklist-items
    pub list: ReviewReqChecklist,
    _phantom: PhantomData<B>,
}
impl<B: Backend> App<B> {
    #[must_use]
    pub fn new(list: ReviewReqChecklist) -> Self {
        Self {
            list,
            _phantom: PhantomData,
        }
    }

    // TODO: de-panic
    #[allow(clippy::missing_panics_doc)]
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
        let info = if self.list.index == 0 {
            let paras: DiagParagraph = self.list.cargo_status.1.clone().into();
            paras.0
        } else {
            Paragraph::new::<&str>(&self.list.project_todo.1)
        };

        f.render_widget(info.block(block), chunks[1]);
    }

    #[must_use]
    pub fn lines(&self) -> Vec<ListItem> {
        // let tick = "✓";
        let cross = "×";
        let span = |fill| -> Vec<Span> { ["[", fill, "] - "].into_iter().map(Span::raw).collect() };

        let mut line1 = span(cross);
        line1.push(Span::raw(self.list.cargo_status.0.clone()));
        let mut line1 = Line::from(line1);
        if self.list.index == 0 {
            line1.patch_style(Style::default().add_modifier(Modifier::BOLD));
        }
        let mut line2 = span(cross);
        line2.push(Span::raw(self.list.project_todo.0.clone()));
        let mut line2 = Line::from(line2);
        if self.list.index == 1 {
            line2.patch_style(Style::default().add_modifier(Modifier::BOLD));
        }
        [line1, line2].map(ListItem::new).to_vec()
    }
}
