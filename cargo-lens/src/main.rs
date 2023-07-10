#![allow(clippy::uninlined_format_args)]
#![allow(clippy::module_name_repetitions)]
#![warn(unused_crate_dependencies)]

use cargo_metadata::diagnostic::Diagnostic;
use crossbeam::channel::Receiver;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};

mod actor;
pub mod app;
pub mod components;
#[cfg(feature = "debug_socket")]
mod debug;
mod events;
/// Overrides std-provided print macros so it doesn't interfere with the terminal.
/// To use the std-print macros, call with `std::[e]print[ln]!`
mod print_macros;
mod review_req_checklist;

use actor::cargo::{CargoActor, CargoImport};
use app::App;
use events::{select_event, AsyncNtfn, QueueEvent};

use crate::review_req_checklist::ReviewReqChecklist;

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

    let _msg = <CargoActor as CargoImport>::fetch();
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
    let list = ReviewReqChecklist::new(review_req_checklist::foo_bar_items());
    let app = App::new(list);
    let res = event_loop(&mut terminal, app);

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

fn event_loop<B: Backend>(terminal: &mut Terminal<B>, mut app: App<B>) -> io::Result<()> {
    let (cargo_rx, xterm_event_rx) = start_actors();
    terminal.draw(|f| app.render(f))?;

    // TODO: set things up so redraw only when necisary.
    // TODO: fully drain the event queue on each iteration
    loop {
        match select_event::<CargoActor>(&xterm_event_rx, &cargo_rx).expect("todo...") {
            QueueEvent::AsyncEvent(AsyncNtfn::Cargo(ntfn)) => app.list.set_cargo_ntfn(ntfn),
            QueueEvent::AsyncEvent(AsyncNtfn::_App(_app)) => todo!(),
            QueueEvent::InputEvent(Ok(Event::Key(k))) => match k.code {
                event::KeyCode::Up => {
                    app.list.up();
                }
                event::KeyCode::Down => {
                    app.list.down();
                }
                event::KeyCode::Tab => {
                    app.list.toggle();
                }
                event::KeyCode::Char('q') => break,
                _ => continue,
            },
            QueueEvent::InputEvent(_) => continue,
        };
        terminal.draw(|f| app.render(f))?;
    }
    Ok(())
}

#[allow(clippy::type_complexity)]
fn start_actors() -> (
    Receiver<Result<Vec<Diagnostic>, <CargoActor as CargoImport>::Error>>,
    Receiver<std::io::Result<Event>>,
) {
    let (cargo_tx, cargo_rx) = crossbeam::channel::unbounded::<
        Result<Vec<Diagnostic>, <CargoActor as CargoImport>::Error>,
    >();
    let (xterm_event_tx, xterm_event_rx) =
        crossbeam::channel::unbounded::<std::io::Result<Event>>();

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
        .name("cargo-fetcher".to_string())
        .spawn(move || {
            let res = <CargoActor as CargoImport>::fetch();
            cargo_tx
                .send(res)
                .expect("todo: handle actor channel fail story");
            loop {
                // TODO?: have a receiver to request a new diagnostic from cargo?
                std::thread::park();
            }
        })
        .unwrap();
    (cargo_rx, xterm_event_rx)
}
