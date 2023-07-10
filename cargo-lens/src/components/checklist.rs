use cargo_metadata::diagnostic::{Diagnostic, DiagnosticLevel};
use ratatui::style::Color;

#[derive(Debug)]
pub struct ReviewReqChecklist {
    pub cargo_status: (String, Option<Vec<Diagnostic>>),
    pub project_todo: (String, String),
    pub index: usize,
}

impl ReviewReqChecklist {
    #[must_use]
    pub fn dev_new() -> Self {
        Self {
            cargo_status: ("cargo status: ".to_string(), None),
            project_todo: foo_bar_items(),
            index: 0,
        }
    }

    pub fn down(&mut self) -> bool {
        if self.index < 1 {
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

    #[must_use]
    pub fn info(&self) -> Option<&String> {
        if self.index == 0 {
            Some(&self.cargo_status.0)
        } else {
            let res = &self.project_todo.1;
            Some(res)
        }
    }
    // pub fn toggle(&mut self) {
    //     let item = if self.index == 0 {
    //         &mut self.cargo_status.2
    //     } else {
    //         &mut self
    //             .items
    //             .get_mut(self.index - 1)
    //             .expect("malformed index")
    //             .toggled
    //     };

    //     *item = !*item;
    // }
    pub fn set_cargo_ntfn(&mut self, state: Vec<Diagnostic>) {
        self.cargo_status.1 = Some(state);
    }
    #[must_use]
    pub fn cargo_color(&self) -> Color {
        let Some(ref diags) = self.cargo_status.1 else {return Color::Gray;};

        let mut res = Color::Green;
        for diag in diags {
            if diag.level == DiagnosticLevel::Error {
                return Color::Red;
            } else if diag.level == DiagnosticLevel::Warning {
                res = Color::Yellow;
            }
        }
        res
    }
}

#[derive(Default, Debug)]
pub struct ReviewReqChecklistItem {
    pub name: String,
    pub info: String,
    pub toggled: bool,
}

#[must_use]
pub fn foo_bar_items() -> (String, String) {
    (
        "todo: lints".to_string(),
        "TODO: toggle tree to reveal available lints, which can be toggled,
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
  [×] suspicious"
            .to_string(),
    )
}
