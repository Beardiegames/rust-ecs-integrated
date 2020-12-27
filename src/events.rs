
use crate::pool::Pointer;

/// An example on how to predefine events
///
#[derive(Clone)]
pub enum ExampleEvents {
    Damage(u16),
    Heal(u16),
    Say(String),
}

/// EventHook is an object that holds information on which Entity the event is 
/// for and what event that Entity has to handle
/// 
#[derive(Clone)]
pub struct EventHook<Events: Clone> {
    pub sender: Pointer,
    pub receiver: Pointer,
    pub event: Events,
}

/// Messenger used to communicate between entities by throwing an event from one 
/// entity to another. See Components.event_handler() trait implementation on 
/// capturing events.
///  
pub struct Messenger<Events: Clone> (Vec<EventHook<Events>>);

impl<Events: Clone> Messenger<Events> {

    pub fn new() -> Self { Messenger (Vec::new()) }

    pub fn tell(&mut self, sender: Pointer, receiver: Pointer, event: Events) {
        self.0.push(EventHook { sender, receiver, event })
    }

    pub fn clear(&mut self) { self.0.clear() }

    pub fn list(&self) -> &Vec<EventHook<Events>> { &self.0 }
}
