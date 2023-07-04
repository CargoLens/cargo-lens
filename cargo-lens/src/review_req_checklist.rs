#[derive(Debug)]
pub struct ReviewReqChecklist<const LEN: usize> {
    pub items: [ReviewReqChecklistItem; LEN],
    pub index: usize,
}

impl<const LEN: usize> ReviewReqChecklist<LEN> {
    pub fn new(items: [ReviewReqChecklistItem; LEN]) -> Self {
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
                name: "First item".to_string(),
                info: "right now, this just demos one part of this checklist idea".to_string(),
                toggled: false,
            },
            ReviewReqChecklistItem {
                name: "todo: make togglable to show done".to_string(),
                info: "helloooooooooo".to_string(),
                toggled: false,
            },
            ReviewReqChecklistItem {
                name: "Buzz".to_string(),
                info: "could be any multiple of 5".to_string(),
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
