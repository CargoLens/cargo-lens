use crossbeam::channel::Sender;
use crossterm::event::Event;

pub struct _InputActor {
    sender: Sender<Event>,
}
