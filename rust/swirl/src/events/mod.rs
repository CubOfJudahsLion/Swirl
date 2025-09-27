//! Module  :   Events
//! Purpose :   A simplified event system. As is, only interactive events are implemented,
//!             but the system is extensible and the user can create his/her own events
//!             and handlers. At some point, a Functional Reactive Programming (FRP) API
//!             will be added as well.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue 04/13/2021 03:48 PM
//! Modified:   Fri 12/10/2021 09:32 AM


use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsValue;
use web_sys::console::error_1;

pub mod interactive;
pub use interactive::*;

pub mod generic;
pub use generic::*;


/// In absence of time implementations in wasm32-unknown-unknown,
/// we'll use JavaScript types.
pub type Duration = u32;


/// Allows a listener to dictate what action should occurr
/// after it receives an event signal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionAfterEvent {
    //  Do nothing special; stay listening.
    NoAction,
    //  Remove itself as a listener.
    RemoveListener,
    //  Do no listen to events for a given time.
    Sleep(Duration),
    //  Skip a number of events.
    Skip(u64),
}


/// Anything that is like an event, offering a type id.
pub trait EventLike {
    /// Selectable event type identifier type.
    type EventIdType: PartialEq + Copy;
    /// Return event identifier.
    fn type_(&self) -> Self::EventIdType;
}

/// An event emitter.
pub trait EventEmitter<Event: EventLike, ListenerId: Copy> {
    fn emit(&mut self, ev: &Event);

    fn subscribe<F: FnMut(&Event) -> ActionAfterEvent + 'static>(&mut self, ev_type: Event::EventIdType, listener: F) -> Option<ListenerId>;

    fn subscribe_multiple<F>(&mut self, ev_types: &[Event::EventIdType], listener: F) -> Vec<(Event::EventIdType, ListenerId)>
        where   F: for<'a> FnMut(&'a Event) -> ActionAfterEvent + 'static
    {
        let mut types           = ev_types.to_vec();
        let mut subscriptions   = Vec::with_capacity(ev_types.len());
        let listener_rc         = Rc::new(RefCell::new(listener));
        types.dedup();
        for type_ in types {
            let listener_rc_clone   = listener_rc.clone();
            if let Some(sub_id) =
                self.subscribe(
                    type_,
                    move |ev| {
                        let action = listener_rc_clone
                            .try_borrow_mut()
                            .map(|mut f| f(ev))
                            .map_err(|_| error_1(&JsValue::from_str("Could not mutably-borrow multiple-subscriber for call.")));
                        if let Ok(action_inner) = action {
                            action_inner
                        }
                        else {
                            ActionAfterEvent::NoAction
                        }
                    }
                )
            {
                subscriptions.push((type_, sub_id));
            }
        }
        subscriptions
    }

    fn unsubscribe(&mut self, ev_type: Event::EventIdType, listener_id: ListenerId) -> bool;

    fn unsubscribe_multiple(&mut self, ev_types: &[(Event::EventIdType, ListenerId)]) -> Vec<bool> {
        ev_types
            .iter()
            .map(|(evt, id)| self.unsubscribe(*evt, *id))
            .collect::<Vec<_>>()
    }

    fn enabled(&self) -> bool;

    fn set_enabled(&mut self, enabled: bool);
}
