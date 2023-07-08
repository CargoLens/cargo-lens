use std::marker::PhantomData;

use cargo_metadata::diagnostic::{Diagnostic, DiagnosticLevel};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
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
            let paras: Vec<Paragraph> = self
                .list
                .cargo_status
                .1
                .iter()
                .cloned()
                .map(|d| DiagParagraph::from(d).0)
                .collect();
            paras
                .get(0)
                .unwrap_or(&Paragraph::new::<&str>("nothing from cargo..."))
                .clone()
                .block(block)
        } else {
            Paragraph::new::<&str>(self.list.info().expect("internal error")).block(block)
        };

        f.render_widget(info, chunks[1]);
    }

    #[must_use]
    pub fn lines(&self) -> Vec<ListItem> {
        let tick = "✓";
        let cross = "×";
        let span = |fill| -> Vec<Span> { ["[", fill, "] - "].into_iter().map(Span::raw).collect() };

        let lines: Vec<ListItem> =
            std::iter::once((&self.list.cargo_status.0, self.list.cargo_status.2))
                .chain(self.list.items.iter().map(|it| (&it.info, it.toggled)))
                .enumerate()
                .map(|(i, (name, toggled))| {
                    let mut spans = if toggled { span(tick) } else { span(cross) };
                    spans.push(Span::raw(name));
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

// get arround the orphan rule
struct DiagParagraph<'a>(Paragraph<'a>);
impl From<Diagnostic> for DiagParagraph<'_> {
    fn from(value: Diagnostic) -> Self {
        let mut spans = vec![];

        let (level, color) = match value.level {
            DiagnosticLevel::Error => ("error", Color::Red),
            DiagnosticLevel::Warning => ("warning", Color::Yellow),
            _ => ("info", Color::White),
        };

        // Create the heading for the diagnostic
        let heading = Span::styled(
            format!("{}: {}", level, value.message),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        );
        spans.push(Line::from(heading));

        // If there is a code snippet, add it
        if let Some(code) = value.code.as_ref() {
            let code_span = Span::styled(
                format!("  --> {}\n", code.code),
                Style::default().fg(Color::Green),
            );
            spans.push(Line::from(code_span));
        }

        // Include each rendered line of the diagnostic message
        for line in value.rendered.unwrap().lines() {
            let line_span = Span::styled(
                format!("     {}\n", line),
                Style::default().fg(Color::Yellow),
            );
            spans.push(Line::from(line_span));
        }

        let text: Text = spans.into();
        let para = Paragraph::new(text);
        Self(para)
    }
}
