use crossbeam::channel::{Receiver, RecvError, Select};
use crossterm::event::{Event, KeyCode};
use non_empty_vec::NonEmpty;

use crate::cargo_interface::{AsyncCargoNtfn, CargoImport, RankedDiagnostic};

pub enum QueueEvent {
    AsyncEvent(AsyncNtfn),
    InputEvent(std::io::Result<Event>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxHighlightProgress {
    Progress,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncAppNtfn {
    SyntaxHighlighting(SyntaxHighlightProgress),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncNtfn {
    App(AsyncAppNtfn),
    Cargo(AsyncCargoNtfn),
}

pub enum SelectError {
    Error,
}

#[derive(Clone, Copy, PartialEq)]
enum Updater {
    Ticker,
    NotifyWatcher,
}

pub fn select_event<D: CargoImport>(
    input_rx: &Receiver<std::io::Result<Event>>,
    diagnostics_rx: &Receiver<Result<Vec<RankedDiagnostic>, D::Error>>,
) -> Result<NonEmpty<QueueEvent>, RecvError> {
    let mut sel = Select::new();

    sel.recv(input_rx);
    sel.recv(diagnostics_rx);

    let mut res: Option<NonEmpty<_>> = None;
    while res.is_none() {
        let oper = sel.select();
        let index = oper.index();

        let ev = match index {
            0 => oper.recv(input_rx).map(QueueEvent::InputEvent),
            1 => oper
                .recv(diagnostics_rx)
                .map(|e| QueueEvent::AsyncEvent(AsyncNtfn::Cargo((&e.unwrap()).into()))),
            _ => continue,
        }?;
        match res {
            Some(ref mut list) => list.push(ev),
            None => res = Some(NonEmpty::new(ev)),
        }
    }

    Ok(res.unwrap())
}
