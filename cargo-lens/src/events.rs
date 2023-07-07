use crossbeam::channel::{Receiver, RecvError, Select};
use crossterm::event::Event;
use non_empty_vec::NonEmpty;

use crate::actor::cargo::{AsyncCargoNtfn, CargoImport, RankedDiagnostic};

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
    Cargo(AsyncCargoNtfn),
}

pub enum _SelectError {
    Error,
}

#[derive(Clone, Copy, PartialEq)]
enum Updater {
    _Ticker,
    _NotifyWatcher,
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
            0 => {
                let Ok(ev) = oper.recv(input_rx) else {continue};
                let Ok(ev) = ev else {continue};
                QueueEvent::InputEvent(Ok(ev))
            }
            1 => {
                let Ok(Ok(ev)) = oper.recv(diagnostics_rx) else {continue};
                let ntfn: AsyncCargoNtfn = (&ev).into();
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
