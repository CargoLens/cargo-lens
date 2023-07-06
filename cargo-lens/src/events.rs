use crossbeam::channel::{Receiver, Select};
use crossterm::event::KeyCode;
use non_empty_vec::NonEmpty;

use crate::diagnostics::{DiagnosticImport, RankedDiagnostic};

///
#[derive(Clone)]
pub enum QueueEvent {
    Tick,
    Notify,
    SpinnerUpdate,
    AsyncEvent(AsyncNtfn),
    InputEvent(KeyCode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxHighlightProgress {
    Progress,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncAppNtfn {
    ///
    SyntaxHighlighting(SyntaxHighlightProgress),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncNtfn {
    ///
    App(AsyncAppNtfn),
    ///
    Cargo(AsyncCargoNtfn),
}

pub enum SelectError {
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsyncCargoNtfn;

#[derive(Clone, Copy, PartialEq)]
enum Updater {
    Ticker,
    NotifyWatcher,
}

fn select_event<D: DiagnosticImport>(
    input_rx: &Receiver<KeyCode>,
    diagnostics_rx: &Receiver<Result<Vec<RankedDiagnostic>, D::Error>>,
) -> Result<NonEmpty<QueueEvent>, SelectError> {
    let mut sel = Select::new();

    sel.recv(input_rx);
    sel.recv(diagnostics_rx);

    let oper = sel.select();
    let index = oper.index();

    let ev = match index {
        0 => oper.recv(input_rx).map(QueueEvent::InputEvent),
        1 => oper
            .recv(diagnostics_rx)
            .map(|e| QueueEvent::AsyncEvent(AsyncNtfn::Cargo(e))),
        _ => panic!("unknown select source"),
    }?;

    Err(todo!())
}
