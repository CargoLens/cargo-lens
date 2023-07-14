use std::cmp::Ordering;

use cargo_metadata::diagnostic::{Diagnostic, DiagnosticLevel};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
};

fn dia_sort(a: &Diagnostic, b: &Diagnostic) -> Ordering {
    let a = a.level;
    let b = b.level;
    if a == b {
        return Ordering::Equal;
    }
    // error -> warn -> rest
    match (a, b) {
        (DiagnosticLevel::Warning, DiagnosticLevel::Error) => Ordering::Greater,
        (DiagnosticLevel::Warning | DiagnosticLevel::Error, _) => Ordering::Less,
        _ => Ordering::Greater,
    }
}

// get arround the orphan rule
pub struct DiagParagraph<'a>(pub Paragraph<'a>);
impl From<Option<Vec<Diagnostic>>> for DiagParagraph<'_> {
    fn from(values: Option<Vec<Diagnostic>>) -> Self {
        let Some(mut values) = values else {
            return Self(Paragraph::new("waiting..."));
        };
        let mut spans = vec![];

        values.sort_by(dia_sort);
        for value in values {
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
            for span in value.spans {
                let code_span = Span::raw(format!(
                    "  --> {}:{}:{}\n",
                    span.file_name, span.line_start, span.column_start
                ));
                spans.push(Line::from(code_span));
            }
        }
        let text: Text = spans.into();
        let para = Paragraph::new(text);
        Self(para)
    }
}
