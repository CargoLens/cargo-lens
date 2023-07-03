#[derive(Debug)]
struct ReviewReqChecklist<const LEN: usize> {
    items: [ReviewReqChecklistItem; LEN],
    index: usize,
}

impl<const LEN: usize> ReviewReqChecklist<LEN> {
    fn new(items: [ReviewReqChecklistItem; LEN]) -> Self {
        Self { items, index: 0 }
    }

    fn down(&mut self) -> bool {
        if self.index < LEN - 1 {
            self.index += 1;
            true
        } else {
            false
        }
    }
    fn up(&mut self) -> bool {
        if self.index > 0 {
            self.index -= 1;
            true
        } else {
            false
        }
    }
}

#[derive(Default, Debug)]
struct ReviewReqChecklistItem {
    name: String,
    info: String,
    toggled: bool,
}

fn foo_bar_list() -> ReviewReqChecklist<3> {
    ReviewReqChecklist {
        items: [
            ReviewReqChecklistItem {
                name: "Fizz".to_string(),
                info: "could be any multiple of 3".to_string(),
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
