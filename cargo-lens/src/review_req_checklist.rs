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
