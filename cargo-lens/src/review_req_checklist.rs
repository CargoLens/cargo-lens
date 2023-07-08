use cargo_metadata::diagnostic::Diagnostic;

#[derive(Debug)]
pub struct ReviewReqChecklist {
    pub cargo_status: (String, Vec<Diagnostic>, bool),
    pub items: Vec<ReviewReqChecklistItem>,
    pub index: usize,
}

impl ReviewReqChecklist {
    pub fn new(items: Vec<ReviewReqChecklistItem>) -> Self {
        Self {
            cargo_status: ("cargo status: ".to_string(), vec![], false),
            items,
            index: 0,
        }
    }

    pub fn down(&mut self) -> bool {
        if self.index < self.items.len() {
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

    pub fn info(&self) -> Option<&String> {
        if self.index == 0 {
            Some(&self.cargo_status.0)
        } else {
            let res = self.items.get(self.index - 1).map(|ent| &ent.info);
            debug_assert!(res.is_some(), "list index outside of indexable range");
            res
        }
    }
    pub fn toggle(&mut self) {
        let item = if self.index == 0 {
            &mut self.cargo_status.2
        } else {
            &mut self
                .items
                .get_mut(self.index - 1)
                .expect("malformed index")
                .toggled
        };

        *item = !*item;
    }
    pub fn set_cargo_ntfn(&mut self, state: Vec<Diagnostic>) {
        self.cargo_status.1 = state
    }
}

#[derive(Default, Debug)]
pub struct ReviewReqChecklistItem {
    pub name: String,
    pub info: String,
    pub toggled: bool,
}

pub fn foo_bar_items() -> Vec<ReviewReqChecklistItem> {
    vec![
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
        ]
}
