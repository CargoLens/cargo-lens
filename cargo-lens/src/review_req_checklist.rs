

#[derive(Debug)]
pub struct ReviewReqChecklist {
    pub items: Vec<ReviewReqChecklistItem>,
    pub index: usize,
}

impl ReviewReqChecklist {
    pub fn _new(items: Vec<ReviewReqChecklistItem>) -> Self {
        Self { items, index: 0 }
    }

    pub fn down(&mut self) -> bool {
        if self.index < self.items.len() - 1 {
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
}

#[derive(Default, Debug)]
pub struct ReviewReqChecklistItem {
    pub name: String,
    pub info: String,
    pub toggled: bool,
}

pub fn foo_bar_list() -> ReviewReqChecklist {
    ReviewReqChecklist {
        items: vec![
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
