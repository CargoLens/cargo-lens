use cargo_metadata::diagnostic::Diagnostic;
use crossbeam::channel::{Receiver, RecvError, Select};
use crossterm::event::Event;
use non_empty_vec::NonEmpty;

use crate::actor::cargo::CargoImport;

pub enum QueueEvent {
    AsyncEvent(AsyncNtfn),
    InputEvent(std::io::Result<Event>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyntaxHighlightProgress {
    _Progress,
    _Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncAppNtfn {
    _SyntaxHighlighting(SyntaxHighlightProgress),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncNtfn {
    _App(AsyncAppNtfn),
    Cargo(Vec<Diagnostic>),
}

pub enum _SelectError {
    Error,
}

#[derive(Clone, Copy, PartialEq)]
enum Updater {
    _Ticker,
    _NotifyWatcher,
}

// TODO: sane-ify this return type
// TODO: make GH issue discussing error-handling pattern in the actor-channel flow
#[allow(clippy::unnecessary_wraps)]
pub fn _select_events<D: CargoImport>(
    input_rx: &Receiver<std::io::Result<Event>>,
    diagnostics_rx: &Receiver<Result<Vec<Diagnostic>, D::Error>>,
) -> Result<NonEmpty<QueueEvent>, RecvError> {
    let mut sel = Select::new();

    sel.recv(input_rx);
    sel.recv(diagnostics_rx);

    let mut res: Option<NonEmpty<_>> = None;
    while res.is_none() {
        let oper = sel.select();
        let index = oper.index();

        let ev = match index {
            0 => {
                let Ok(ev) = oper.recv(input_rx) else {continue};
                let Ok(ev) = ev else {continue};
                QueueEvent::InputEvent(Ok(ev))
            }
            1 => {
                let Ok(Ok(ev)) = oper.recv(diagnostics_rx) else {continue};
                QueueEvent::AsyncEvent(AsyncNtfn::Cargo(ntfn))
            }
            _ => continue,
        };
        match res {
            Some(ref mut list) => list.push(ev),
            None => res = Some(NonEmpty::new(ev)),
        }
    }

    Ok(res.unwrap())
}

pub fn select_event<D: CargoImport>(
    input_rx: &Receiver<std::io::Result<Event>>,
    diagnostics_rx: &Receiver<Result<Vec<Diagnostic>, D::Error>>,
) -> Result<QueueEvent, RecvError> {
    let mut sel = Select::new();

    sel.recv(input_rx);
    sel.recv(diagnostics_rx);

    let oper = sel.select();
    let index = oper.index();

    let ev = match index {
        0 => QueueEvent::InputEvent(oper.recv(input_rx)?),
        1 => {
            let ev = oper
                .recv(diagnostics_rx)?
                .expect("toodo: handle cargo import error when selecting event");
            QueueEvent::AsyncEvent(AsyncNtfn::Cargo(ntfn))
        }
        _ => todo!("handle other event/error"),
    };
    Ok(ev)
}
