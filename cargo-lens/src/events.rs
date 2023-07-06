///
#[derive(Clone)]
pub enum QueueEvent {
    Tick,
    Notify,
    SpinnerUpdate,
    AsyncEvent(AsyncNotification),
    InputEvent(InputEvent),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxHighlightProgress {
    Progress,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncAppNotification {
    ///
    SyntaxHighlighting(SyntaxHighlightProgress),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncNotification {
    ///
    App(AsyncAppNotification),
    ///
    Git(AsyncGitNotification),
}

#[derive(Clone, Copy, PartialEq)]
enum Updater {
    Ticker,
    NotifyWatcher,
}
