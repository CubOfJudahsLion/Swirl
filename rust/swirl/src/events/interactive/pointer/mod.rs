//! Module  :   Pointer Events
//! Purpose :   Mouse and Touch events, translated into a single tipe
//!             of event: a ```PointerEvent```, as implemented below.
//! Version :   0.1.3
//! Author  :   Alex Feterman
//! Created :   Thu Feb 25 03:21:53 PM CST 2021
//! Modified:   Tue 11/02/2021 10:50 AM


use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

use crate::math::numeric::Floating;
use crate::math::geom::rect::RectBounds;
use crate::utils::{shared::Shared, enum_set::make_enum_set};
use crate::scene_graph::SceneGraph;
use super::super::{EventLike, EventEmitter, generic::GenericEmitter};
use super::super::super::{Point};
use super::{Modifiers, UIEventLike};

//  The "event driver" for mouse events.
pub mod driver;
pub use driver::*;


//  A list of browser event names.
pub(crate) mod browser_ev_names {
    pub const CLICK         : &'static str   = "click";
    pub const DBLCLICK      : &'static str   = "dblclick";
    pub const MOUSEMOVE     : &'static str   = "mousemove";
    pub const MOUSEDOWN     : &'static str   = "mousedown";
    pub const MOUSEUP       : &'static str   = "mouseup";
    pub const MOUSEENTER    : &'static str   = "mouseenter";
    pub const MOUSELEAVE    : &'static str   = "mouseleave";
    pub const TOUCHSTART    : &'static str   = "touchstart";
    pub const TOUCHMOVE     : &'static str   = "touchmove";
    pub const TOUCHEND      : &'static str   = "touchend";
    pub const TOUCHCANCEL   : &'static str   = "touchcancel";
}


/// Pointer events are UI events with coordinates.
pub trait PointerEventLike<N>: UIEventLike {
    fn pos(&self) -> &Point<N>;
}

//  Identifies the kind of pointer event.
make_enum_set!{
    PointerEventType ::=
        Move        => "Move",
        Enter       => "Enter",
        Leave       => "Leave",
        Down        => "Down",
        Up          => "Up",
        Click       => "Click",
        DblClick    => "DblClick",
}


impl TryFrom<&str> for PointerEventType {
    /// There's only one kind of error that can occur:
    /// the string is not the name of a pointer event.
    type Error = ();

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        use browser_ev_names::*;
        use PointerEventType::*;
        match name {
            CLICK       =>  Ok(Click),
            DBLCLICK    =>  Ok(DblClick),
            TOUCHSTART  |
            MOUSEDOWN   =>  Ok(Down),
            MOUSEENTER  =>  Ok(Enter),
            MOUSELEAVE  =>  Ok(Leave),
            TOUCHMOVE   |
            MOUSEMOVE   =>  Ok(Move),
            TOUCHEND    |
            MOUSEUP     =>  Ok(Up),
            _           =>  Err(())
        }
    }
}


/// The pointer event proper.
#[derive(Clone, Debug)]
pub struct PointerEvent<N> {
    type_   : PointerEventType,
    pos     : Point<N>,
    mods    : Modifiers,
}

impl<N> PointerEvent<N> {
    /// Create from Copy or reference types.
    pub fn new(type_: PointerEventType, pos: &Point<N>, mods: &Modifiers) -> Self
        where N: Clone
    {
        PointerEvent {
            type_,
            pos     : pos.clone(),
            mods    : mods.clone(),
        }
    }

    //  Create from a browser event, which provides the data but not the event type
    //  (the latter is passed as a parameter.) Since we implement our own component
    //  events, we might have an m:n browser event/local event correspondence, and
    //  not always including the identity.
    pub(crate) fn from_browser_mouse_event(
        browser_ev  : &web_sys::MouseEvent,
        type_       : PointerEventType,
        pos         : &Point<N>
    ) -> Self
        where N: Copy
    {
        PointerEvent {
            type_,
            pos     :   pos.clone(),
            mods    :   Modifiers::from_mouse_event_modifiers(browser_ev),
        }
    }
}

impl<N> EventLike for PointerEvent<N> {
    type EventIdType = PointerEventType;
    fn type_(&self) -> Self::EventIdType { self.type_ }
}

impl<N> UIEventLike for PointerEvent<N> {
    fn modifiers(&self) -> &Modifiers { &self.mods }
}

impl<N> PointerEventLike<N> for PointerEvent<N> {
    fn pos(&self) -> &Point<N> { &self.pos }
}


//  We need an emitter for pointer events. We're basically using containment --
//  wrapping one around a generic emitter with the right types. We're also
//  using a `SceneGraph` element to delimit our area of interest.
struct PointerEventEmitterInner<N: 'static> {
    pub(crate)  emitter     : GenericEmitter<PointerEvent<N>>,
    pub(crate)  node        : Box<dyn SceneGraph<N>>,
    pub(crate)  was_down    : bool,
}


/// A handle to a pointer events emitter.
#[derive(Clone)]
pub struct PointerEventEmitter<N: 'static>(Shared<PointerEventEmitterInner<N>>);


impl<N> PointerEventEmitter<N>
    where   N   :   Copy
                +   Floating
                +   RectBounds
                +   From<i16>
                +   'static
{
    /// Create a new PointerEventEmitter for a given
    /// `SceneGraph`-implementing element.
    pub(crate) fn new_dyn(node: Box<dyn SceneGraph<N>>) -> Self {
        PointerEventEmitter(Shared::new(PointerEventEmitterInner {
            emitter     : GenericEmitter::<PointerEvent<N>>::new(),
            node,
            was_down    : false,
        }))
    }

    #[inline(always)]
    pub(crate) fn new<S: SceneGraph<N> + Clone + 'static>(node: &S) -> Self {
        Self::new_dyn(Box::new(node.clone()) as Box<dyn SceneGraph<N>>)
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.0.emitter.set_enabled(enabled);
        if !enabled {
            self.0.was_down = false;
        }
    }
}


impl<N: 'static> Deref for PointerEventEmitter<N> {
    type Target = GenericEmitter<PointerEvent<N>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0.emitter
    }
}

impl<N: 'static> DerefMut for PointerEventEmitter<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.emitter
    }
}
