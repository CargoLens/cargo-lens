#![warn(unused_crate_dependencies)]

use crossbeam::channel::Receiver;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use diagnostics::RankedDiagnostic;
use non_empty_vec::NonEmpty;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io};

#[cfg(feature = "debug_socket")]
mod debug;
mod diagnostics;
mod events;
/// Overrides std-provided print macros so it doesn't interfere with the terminal.
/// To use the std-print macros, call with `std::[e]print[ln]!`
mod print_macros;
mod review_req_checklist;

use crate::diagnostics::{CargoDispatcher, DiagnosticImport};
use events::*;

fn main() -> Result<(), Box<dyn Error>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "debug_socket")] {
            std::println!("listening on localhost:8080 for external debugger connection...");
            let _debug_out = { debug::connect_to_iface() }?;
            std::println!("`println!` disabled. stdout redirected to localhost:8080");
        } else {
            std::println!("println! and eprintln! disabled. stdout and the tui are now in an exclusive relationship.");
        }
    }

    let _msg = <CargoDispatcher as DiagnosticImport>::fetch();
    // Give something to diagnose: debugger should see a warning
    #[cfg(feature = "debug_socket")]
    {
        let foo = 1;
        eprintln!("{:#?}", _msg);
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = event_loop(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        std::eprintln!("{:?}", err);
    }

    Ok(())
}

fn event_loop<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut list = review_req_checklist::foo_bar_list();

    let (xterm_event_tx, xterm_event_rx) =
        crossbeam::channel::unbounded::<std::io::Result<Event>>();
    let (diagnostics_tx, diagnostics_rx) = crossbeam::channel::unbounded::<
        Result<Vec<RankedDiagnostic>, <CargoDispatcher as DiagnosticImport>::Error>,
    >();

    std::thread::Builder::new()
        .name("crossterm-event-reader".to_string())
        .spawn(move || loop {
            // TODO: a story to unblock + shutdown gracefully
            xterm_event_tx
                .send(event::read())
                .expect("todo: handle actor channel fail story");
        })
        .unwrap();
    std::thread::Builder::new()
        .name("diagnostic-fetcher".to_string())
        .spawn(move || {
            loop {
                // TODO?: have a receiver to request a new diagnostic?
                let res = <CargoDispatcher as diagnostics::DiagnosticImport>::fetch();
                diagnostics_tx
                    .send(res)
                    .expect("todo: handle actor channel fail story");
            }
        })
        .unwrap();

    loop {
        let mut redraw = false;

        // While there are messages on any channel, handle them and set redraw to true
        loop {
            /* redraw |= */
            crossbeam::channel::select! {
                recv(diagnostics_rx) -> diagnostics => todo!("handle diagnostics"),
                recv(xterm_event_rx) -> key_event => todo!("hondle key event"),
                default => break,
            };
            // match event {
            //     Some(Event::Diagnostic(diagnostics)) => {
            //         // Process diagnostics
            //         redraw = true;
            //     }
            //     Some(Event::Key(key_event)) => {
            //         // Process key event
            //         redraw = true;
            //     }
            //     None => {
            //         // No more messages, break the loop
            //         break;
            //     }
            // }
        }

        if redraw {
            terminal.draw(|f| ui(f, &mut list))?;
        }
    }
}

fn select_event<D: DiagnosticImport>(
    input_rx: Receiver<KeyCode>,
    diagnostics_rx: Receiver<Result<Vec<RankedDiagnostic>, D::Error>>,
) -> NonEmpty<QueueEvent> {
    todo!()
}
fn ui<const LEN: usize, B: Backend>(
    f: &mut Frame<B>,
    list: &mut review_req_checklist::ReviewReqChecklist<LEN>,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Min(0)].as_ref())
        .split(f.size());
    let items: Vec<_> = list
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            // TODO: think about organising this code nicely. this is rapid-prototype-crap
            // TODO: change between red and green text color instead
            let prefix = if item.toggled { "[✓] - " } else { "[×] - " };
            let res = ListItem::new(format!("{}{}", prefix, item.name));
            if i == list.index {
                // TODO: find a nicer way to highlight
                res.style(Style::default().add_modifier(Modifier::BOLD))
            } else {
                res
            }
        })
        .collect();

    let block = Block::default().title("Checklist").borders(Borders::ALL);
    let checklist = List::new(items).block(block);
    f.render_widget(checklist, chunks[0]);
    let block = Block::default().title("Info").borders(Borders::ALL);
    let info =
        Paragraph::new::<&str>(list.items.get(list.index).unwrap().info.as_ref()).block(block);
    f.render_widget(info, chunks[1]);
}
