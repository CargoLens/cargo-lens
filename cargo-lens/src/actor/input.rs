use crossbeam::channel::Sender;
use crossterm::event::Event;

pub struct InputActor {
    sender: Sender<Event>,
}
