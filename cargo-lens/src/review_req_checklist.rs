use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

#[derive(Debug)]
pub struct ReviewReqChecklist<const LEN: usize> {
    pub items: [ReviewReqChecklistItem; LEN],
    pub index: usize,
}

impl<const LEN: usize> ReviewReqChecklist<LEN> {
    pub fn _new(items: [ReviewReqChecklistItem; LEN]) -> Self {
        Self { items, index: 0 }
    }

    pub fn down(&mut self) -> bool {
        if self.index < LEN - 1 {
            self.index += 1;
            true
        } else {
            false
        }
    }
    pub fn up(&mut self) -> bool {
        if self.index > 0 {
            self.index -= 1;
            true
        } else {
            false
        }
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
                if i == self.index {
                    res.style(Style::default().add_modifier(Modifier::BOLD))
                } else {
                    res
                }
            })
            .collect();
        lines
    }
}

#[derive(Default, Debug)]
pub struct ReviewReqChecklistItem {
    pub name: String,
    pub info: String,
    pub toggled: bool,
}

pub fn foo_bar_list() -> ReviewReqChecklist<4> {
    ReviewReqChecklist {
        items: [
            ReviewReqChecklistItem {
                name: "compiles".to_string(),
                info: "TODO: extract json from compiler. Mark item as complete if good".to_string(),
                toggled: false,
            },
            ReviewReqChecklistItem {
                name: "compiles without warnings".to_string(),
                info: "TODO: extract json from compiler. Do something with list of warnings. have it affect the list item struct (e.g. auto toggle if no warnings)".to_string(),
                toggled: false,
            },
            ReviewReqChecklistItem {
                name: "Lints".to_string(),
                info: "TODO: toggle tree to reveal available lints, which can be toggled,
Each item, when indexed, would show the warnings it coveres
also: think about how to aggregate, list, present warnings?
the list-item might look like this when expanded:

▼ Lint-options
  - 'a' - all, 'd' - default, 'n' - none
  [✓] complexity
  [×] correctness
  [✓] deprecated
  [✓] nursery
  [×] pedantic
  [×] perf
  [×] restriction
  [×] style
  [×] suspicious".to_string(),
                toggled: false,
            },
            ReviewReqChecklistItem {
                name: "FizzBuzz".to_string(),
                info: "could be any shared-multiple of 3 and 5".to_string(),
                toggled: false,
            },
        ],
        index: 0,
    }
}
