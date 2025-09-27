//! Module  :   Events
//! Purpose :   A simplified event system. As is, only interactive events are implemented,
//!             but the system is extensible and the user can create his/her own events
//!             and handlers.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu Feb 25 03:21:53 PM CST 2021
//! Modified:   Tue 04/13/2021 10:57 AM


//
use std::vec::Vec;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;
use std::ops::{Deref, IndexMut};
use std::borrow::BorrowMut;
use std::convert::{AsRef, TryFrom, From};

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use js_sys::{Function, Object};
use web_sys::{self, window, HtmlCanvasElement, TouchEvent, TouchList, Touch};

use crate::make_enum_set;
use super::math::{Point, Rect};
use crate::utils::*;


/// Keyboard modifiers. While not expected to receive much use
/// for games, in-house tools might take advantage of them.
#[derive(Clone, Copy, Debug)]
pub struct Modifiers {
    pub alt     : bool,
    pub ctrl    : bool,
    pub shift   : bool,
    pub meta    : bool,
}

impl Modifiers {
    /// Created directly by the user.
    pub fn new(alt: bool, ctrl: bool, shift: bool, meta: bool) -> Self {
        Modifiers {alt, ctrl, shift, meta,}
    }

    /// Adapt directly from a browser's MouseEvent.
    fn from_mouse_event_modifiers(browser_ev: &web_sys::MouseEvent) -> Self {
        Modifiers {
            alt     : browser_ev.alt_key(),
            ctrl    : browser_ev.ctrl_key(),
            shift   : browser_ev.shift_key(),
            meta    : browser_ev.meta_key(),
        }
    }

    /// Adapt directly from TouchEvent.
    fn from_touch_event_modifiers(browser_ev: &TouchEvent) -> Self {
        Modifiers {
            alt     : browser_ev.alt_key(),
            ctrl    : browser_ev.ctrl_key(),
            shift   : browser_ev.shift_key(),
            meta    : browser_ev.meta_key(),
        }
    }
}

impl Display for Modifiers {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Modifiers{{alt: {}, ctrl: {}, shift: {}}}", self.alt, self.ctrl, self.shift)
    }
}


/// Anything that is like an event, offering a type id.
pub trait EventLike {
    /// Selectable event type identifier type.
    type EventIdType;
    /// Return event identifier.
    fn type_(&self) -> Self::EventIdType;
}

/// An user-interface event, mostly comprised of user actions.
pub trait UIEventLike: EventLike {
    /// Obtain the keyboard modifiers.
    fn modifiers(&self) -> &Modifiers;
}

/// Pointer events are UI events with coordinates.
pub trait PointerEventLike: UIEventLike {
    fn pos(&self) -> &Point<i32>;
}


/// An event emitter.
pub trait EventEmitter<Event: EventLike, ListenerId: Copy> {
    fn emit(&mut self, ev: Event);
    fn subscribe<F: FnMut(&Event) + 'static>(&mut self, ev_type: Event::EventIdType, listener: F) -> Option<ListenerId>;
    fn unsubscribe(&mut self, ev_type: Event::EventIdType, listener_id: ListenerId) -> bool;
    fn set_enabled(&mut self, enabled: bool);
}


//  A structure holding a mutable function and its extracted "hasher" value
struct OrderedFunction<Event>(usize, Box<dyn FnMut(&Event)>);

impl<Event: EventLike> OrderedFunction<Event> {
    #[inline(always)]
    fn address_as_number<F>(f: &F) -> usize
        where F: FnMut(&Event)
    {
        f as *const F as usize
    }

    pub fn new<F>(f: F) -> Self
        where   F: FnMut(&Event) + 'static
    {
        OrderedFunction(Self::address_as_number(&f), Box::new(f) as Box<dyn FnMut(&Event)>)
    }

    #[inline(always)]
    pub fn call(&mut self, ev: &Event) {
        (self.1)(ev);
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.0
    }
}

//impl<Event> PartialEq for OrderedFunction<Event> {
//    fn eq(&self, other: &Self) -> bool {
//        self.0 == other.0
//    }
//}
//
//impl<Event> PartialOrd for OrderedFunction<Event> {
//    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//        self.0.partial_cmp(&other.0)
//    }
//}


/// A generic event emitter type.
pub struct GenericEmitter<Event, Data, Discriminator>
    where   Event               : EventLike,
            Event::EventIdType  : EnumSet,
            Discriminator       : FnMut(&Data, &Event) -> bool + 'static
{
    enabled         : bool,
    data            : Data,
    discriminator   : Discriminator,
    subscribers     : EnumSetIndexed<Event::EventIdType, Vec<OrderedFunction<Event>>>,
}

//  During a search, we could find the datum, or telling where
//  it should be inserted in an ordered vector.
#[derive(Clone, Copy)]
enum BinSearchResult {
    FoundAt(usize),
    CanInsertAt(usize),
}

impl BinSearchResult {
    fn map<F>(&self, f: F) -> Self
        where   F: FnOnce(usize) -> usize
    {
        use BinSearchResult::*;
        match self {
            &FoundAt(n)     => FoundAt(f(n)),
            &CanInsertAt(n) => CanInsertAt(f(n)),
        }
    }
}

impl<Event, Data, Discriminator> GenericEmitter<Event, Data, Discriminator>
    where   Event               : EventLike,
            Event::EventIdType  : EnumSet,
            Discriminator       : FnMut(&Data, &Event) -> bool + 'static
{
    const ESTIMATED_MAX_SUBSCRIBERS: usize = 4;

    pub fn new(data: Data, discriminator: Discriminator) -> Self
    {
        GenericEmitter {
            enabled         : true,
            data,
            discriminator,
            subscribers     :
                EnumSetIndexed::<Event::EventIdType, Vec<OrderedFunction<Event>>>
                    ::new_mapped(
                        &mut |_: &Event::EventIdType|
                            Vec::with_capacity(Self::ESTIMATED_MAX_SUBSCRIBERS)
                    ),
        }
    }

    fn binary_search(id: usize, s: &[OrderedFunction<Event>]) -> BinSearchResult {
        use BinSearchResult::*;
        let s_len = s.len();
        if s_len == 0 {
            CanInsertAt(0)
        }
        else {
            let pivot_pos   = s_len / 2;
            let pivot_id    = s[pivot_pos].id();
            if pivot_id == id {
                FoundAt(pivot_pos)
            }
            else if pivot_id > id {
                Self::binary_search(id, &s[pivot_pos + 1..])
                    .map(|index| index + pivot_pos + 1)
            }
            else {
                Self::binary_search(id, &s[..pivot_pos])
            }
        }
    }
}


impl<Event, Data, Discriminator> EventEmitter<Event, usize> for GenericEmitter<Event, Data, Discriminator>
    where   Event               : EventLike,
            Event::EventIdType  : EnumSet,
            Discriminator       : FnMut(&Data, &Event) -> bool + 'static,
            usize               : From<Event::EventIdType>
{
    fn emit(&mut self, ev: Event) {
        if self.enabled && (self.discriminator)(&self.data, &ev) {
            for listener in self.subscribers.index_mut(ev.type_()) {
                listener.call(&ev);
            }
        }
    }

    fn subscribe<F>(&mut self, ev_type: Event::EventIdType, listener: F) -> Option<usize>
        where   F: FnMut(&Event) + 'static
    {
        let ord_fn  = OrderedFunction::new(listener);
        let ord_id  = ord_fn.id();
        use BinSearchResult::*;
        match GenericEmitter::<Event, Data, Discriminator>
            ::binary_search(ord_id, &self.subscribers[ev_type][..])
        {
            FoundAt(_)      =>
                None,
            CanInsertAt(i)  => {
                self.subscribers[ev_type].insert(i, ord_fn);
                Some(ord_id)
            }
        }
    }

    fn unsubscribe(&mut self, ev_type: Event::EventIdType, listener_id: usize) -> bool {
        let listeners = self.subscribers.index_mut(ev_type);
        use BinSearchResult::*;
        match GenericEmitter::<Event, Data, Discriminator>
            ::binary_search(listener_id, listeners)
        {
            FoundAt(i)      => {
                listeners.remove(i);
                true
            },
            _               =>
                false
        }
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;        
    }
}


//  Identifies the kind of pointer event.
make_enum_set!{
    PointerEventType ::=
        Move        => PointerEvent::MOUSEMOVE,
        Enter       => PointerEvent::MOUSEENTER,
        Leave       => PointerEvent::MOUSELEAVE,
        Down        => PointerEvent::MOUSEDOWN,
        Up          => PointerEvent::MOUSEUP,
        Click       => PointerEvent::CLICK,
        DblClick    => PointerEvent::DBLCLICK,
}


/// Error emitted when the user tries to obtain a `PointerEventType`
/// from string that doesn't match the browser event names.
pub struct EventConversionFromNameError;


impl TryFrom<&str> for PointerEventType {
    type Error = EventConversionFromNameError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        use PointerEventType::*;
        match name {
            PointerEvent::CLICK         =>  Ok(Click),
            PointerEvent::DBLCLICK      =>  Ok(DblClick),
            PointerEvent::TOUCHSTART    |
            PointerEvent::MOUSEDOWN     =>  Ok(Down),
            PointerEvent::MOUSEENTER    =>  Ok(Enter),
            PointerEvent::MOUSELEAVE    =>  Ok(Leave),
            PointerEvent::TOUCHMOVE     |
            PointerEvent::MOUSEMOVE     =>  Ok(Move),
            PointerEvent::TOUCHEND      |
            PointerEvent::MOUSEUP       =>  Ok(Up),
            _                           =>  Err(EventConversionFromNameError)
        }
    }
}


/// The pointer event proper.
#[derive(Clone)]
pub struct PointerEvent {
    type_   : PointerEventType,
    pos     : Point<i32>,
    mods    : Modifiers,
}

impl PointerEvent {
    pub(crate) const CLICK         : &'static str   = "click";
    pub(crate) const DBLCLICK      : &'static str   = "dblclick";
    pub(crate) const MOUSEMOVE     : &'static str   = "mousemove";
    pub(crate) const MOUSEDOWN     : &'static str   = "mousedown";
    pub(crate) const MOUSEUP       : &'static str   = "mouseup";
    pub(crate) const MOUSEENTER    : &'static str   = "mouseenter";
    pub(crate) const MOUSELEAVE    : &'static str   = "mouseleave";
    pub(crate) const TOUCHSTART    : &'static str   = "touchstart";
    pub(crate) const TOUCHMOVE     : &'static str   = "touchmove";
    pub(crate) const TOUCHEND      : &'static str   = "touchend";
    pub(crate) const TOUCHCANCEL   : &'static str   = "touchcancel";


    /// Create from Copy or reference types.
    #[allow(dead_code)]
    pub fn new(type_: PointerEventType, pos: &Point<i32>, mods: &Modifiers) -> Self {
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
    fn from_browser_mouse_event(browser_ev: &web_sys::MouseEvent, type_: PointerEventType, scale: f32) -> Self {
        PointerEvent {
            type_,
            pos     :   Point::new(
                            (browser_ev.offset_x() as f32 * scale) as i32,
                            (browser_ev.offset_y() as f32 * scale) as i32
                        ),
            mods    :   Modifiers::from_mouse_event_modifiers(browser_ev),
        }
    }
}

impl EventLike for PointerEvent {
    type EventIdType = PointerEventType;
    fn type_(&self) -> Self::EventIdType { self.type_ }
}

impl UIEventLike for PointerEvent {
    fn modifiers(&self) -> &Modifiers { &self.mods }
}

impl PointerEventLike for PointerEvent {
    fn pos(&self) -> &Point<i32> { &self.pos }
}


//  Hit areas are complex type-wise. A resource-counted list of event
//  types each holding a vector of callbacks. These gradual types
//  simplify the required signatures.
type EventCallbackList  = Vec<Box<dyn FnMut(&PointerEvent)>>;
type EventHandlers      = [EventCallbackList; PointerEventType::VARIANT_COUNT];
type ShEventHandlers    = Shared<EventHandlers>;


/// Hit areas are separated from `crate::mingly::Drawable`s for now. Defined
/// as event interceptor in a rectangular area, they track both Mouse and Touch
/// events and offer both to the consumer as `PointerEvent`s. Since we create
/// games usable both on mobile and desktop, it makes sense to factorize the
/// capabilities of both, offering a simpler interface.
pub struct HitArea {
    handlers_by_event: ShEventHandlers,
}

impl HitArea {
    //  Dispatches event callbacks to the proper handlers.
    fn dispatch(handlers_by_event: &mut ShEventHandlers, event: &PointerEvent) {
        let handlers    = handlers_by_event[event.type_() as usize];
        for handler in handlers {
            handler(event);
        }
    }

    //  Sets up listening for mouse-specific browser events, relaying
    //  their coordinate-normalized and simplified versions to the hit area.
    fn setup_mouse_handler(
        handlers_by_event   : ShEventHandlers,
        canvas              : Rc<HtmlCanvasElement>,
        hit_area            : Rect<f32>
    ) -> Result<(), JsValue>
    {
        let cnv_width       = canvas.width() as f32;        // Original width of canvas
        let mut prev_pos    = Point::<f32>::new(0.0, 0.0);  // Previous mouse position
        let canvas_copy     = canvas.clone();               // Since the original canvas is left to the closure
        // The closure relays simplified mouse events to the handlers, hence the name.
        let mouse_relay     = Closure::wrap(
            Box::new(
                move |browser_ev: web_sys::MouseEvent| {
                    let scale   = cnv_width / canvas.client_width() as f32;
                    let pos     = Point::new(browser_ev.offset_x() as f32, browser_ev.offset_y() as f32) * scale;
                    let type_   = browser_ev.type_();
                    match &type_[..] {
                        PointerEvent::MOUSEENTER |
                        PointerEvent::MOUSELEAVE |
                        PointerEvent::MOUSEMOVE           => {
                            // Previous position outside, current inside. We entered the hit area.
                            // Emit both a Move and an Enter event.
                            if hit_area.contains(&pos) && !hit_area.contains(&prev_pos) {
                                Self::dispatch(
                                    &mut handlers_by_event,
                                    &PointerEvent::from_browser_mouse_event(
                                        &browser_ev,
                                        PointerEventType::Move,
                                        scale
                                    )
                                );
                                Self::dispatch(
                                    &mut handlers_by_event,
                                    &PointerEvent::from_browser_mouse_event(
                                        &browser_ev,
                                        PointerEventType::Enter,
                                        scale
                                    )
                                );
                            }
                            // The opposite: previous position inside, current one outside.
                            // We just let them know that the mouse left.
                            else if !hit_area.contains(&pos) && hit_area.contains(&prev_pos) {
                                Self::dispatch(
                                    &mut handlers_by_event, 
                                    &PointerEvent::from_browser_mouse_event(
                                        &browser_ev,
                                        PointerEventType::Leave,
                                        scale
                                    )
                                );
                            }
                            //  Any other movement-related event inside the hit area
                            //  is simply reported as motion.
                            else if hit_area.contains(&pos) {
                                Self::dispatch(
                                    &mut handlers_by_event,
                                    &PointerEvent::from_browser_mouse_event(
                                        &browser_ev,
                                        PointerEventType::Move,
                                        scale
                                    )
                                );
                            }
                        },
                        //  All button events: reported if they're inside the
                        //  area and we have a PointerEventType for them.
                        _ if hit_area.contains(&pos)    => {
                            if let Ok(pev) = <PointerEventType as TryFrom<&str>>::try_from(&type_[..]) {
                                Self::dispatch(
                                    &mut handlers_by_event,
                                    &PointerEvent::from_browser_mouse_event(
                                        &browser_ev,
                                        pev,
                                        scale
                                    )
                                );
                            }
                        },
                        _                               => (),
                    }
                    prev_pos.set(pos.x, pos.y);
                }
            ) as Box<dyn FnMut(web_sys::MouseEvent)>
        );
        // Attach the relay to the browser listeners through the JS `Function` interface.
        let mouse_relay_fn  = mouse_relay.as_ref().unchecked_ref::<Function>();
        for mev in &[   PointerEvent::MOUSEMOVE
                    ,   PointerEvent::MOUSEENTER, PointerEvent::MOUSELEAVE
                    ,   PointerEvent::MOUSEUP,    PointerEvent::MOUSEDOWN
                    ,   PointerEvent::CLICK,      PointerEvent::DBLCLICK
                    ]
        {
            canvas_copy.add_event_listener_with_callback(mev, mouse_relay_fn)?;
        }
        mouse_relay.forget();   // This might go away in the future. We'll see.
                                // If we create an App class with a main loop,
                                // we'll just save the closure in the struct itself
                                // and let the App keep that one.
        Ok(())
    }

    //  The functions ahead are Touch event-related.

    //  Extracts a touch with a specific id -- as long as said id
    //  has been provided and it corresponds to one of the touches.
    //  This double-discriminating function simplifies our event
    //  handling patterns below.
    fn extract_touch_from_list(touches: &TouchList, oid: &Option<i32>) -> Result<Touch, JsValue> {
        match oid {
            &None       => Err(JsValue::NULL),
            &Some(id)   => {
                for i in 0..touches.length() {
                    let touch = touches.item(i).unwrap();
                    if touch.identifier() == id {
                        return Ok(touch);
                    }
                }
                Err(JsValue::NULL)
            },
        }
    }

    //  Handles a single touch event conditionally, and reports
    //  on whether or not it was handled. A `true` return value
    //  indicates that it lied inside the hit area and it was
    //  processed; false indicates that it was ignored.
    fn handle_touch_event(
        touch       : &Touch,
        hit_area    : &Rect<f32>,
        canvas      : &HtmlCanvasElement,
        cnv_width   : f32,
        dispatch    : &[PointerEventType],
        handlers    : &ShEventHandlers,
        browser_ev  : &TouchEvent
    ) -> bool
    {
        let canvas_bounds   = canvas.get_bounding_client_rect();
        let scale           = cnv_width / canvas.client_width() as f32;
        let pos             = Point::new(
            touch.client_x() as f32 - canvas_bounds.x() as f32,
            touch.client_y() as f32 - canvas_bounds.y() as f32
        ) * scale;
        if hit_area.contains(&pos) {
            for &ev_type in dispatch {
                Self::dispatch(
                    &mut handlers,
                    &PointerEvent::new(
                        ev_type,
                        &Point::new(pos.x as i32, pos.y as i32),
                        &Modifiers::from_touch_event_modifiers(&browser_ev)
                    )
                );
            }
            true
        }
        else {
            false
        }
    }

    //const DBL_TIME      : Duration      = Duration::from_millis(500);

    //  The counterpart of `setup_mouse_handler` above, it specifically
    //  translates touch into mouse-like events. This limits your interaction
    //  to one finger, but as our games have to work for mouse users as
    //  well, limiting our tracking to a single touch is the logical thing to do.
    fn setup_touch_handler(
        handlers: ShEventHandlers,
        canvas  : Rc<HtmlCanvasElement>,
        hit_area: Rect<f32>
    ) -> Result<(), JsValue>
    {
        let cnv_width                           = canvas.width() as f32;
        let mut tracked_touch   : Option<i32>   = None;
        let canvas_copy                         = canvas.clone();
        let touch_relay                         = Closure::wrap(Box::new(
            move |browser_ev: TouchEvent| {
                browser_ev.stop_immediate_propagation();
                let type_           = browser_ev.type_();
                match &type_[..] {
                    PointerEvent::TOUCHSTART if tracked_touch.is_none() => {
                        let touch       = browser_ev.changed_touches().item(0).unwrap();
                        let dispatch    = [PointerEventType::Enter, PointerEventType::Down];
                        if Self::handle_touch_event(
                            &touch, &hit_area, &canvas, cnv_width, &dispatch[..], &handlers, &browser_ev
                        )
                        {
                            tracked_touch = Some(touch.identifier());
                        }
                    },
                    PointerEvent::TOUCHMOVE                             => {
                        match Self::extract_touch_from_list(&browser_ev.touches(), &tracked_touch) {
                            Ok(touch)   => {
                                let bounds  = canvas.get_bounding_client_rect();
                                let scale   = bounds.width() as f32 / cnv_width;
                                let pos     = Point::<f32>::new(
                                    touch.client_x() as f32 - bounds.x() as f32,
                                    touch.client_y() as f32 - bounds.y() as f32
                                ) * scale;
                                Self::dispatch(
                                    &mut handlers,
                                    &PointerEvent::new(
                                        if hit_area.contains(&pos) {
                                            PointerEventType::Move
                                        }
                                        else {
                                            tracked_touch = None;
                                            PointerEventType::Leave
                                        },
                                        &Point::new(pos.x as i32, pos.y as i32),
                                        &Modifiers::from_touch_event_modifiers(&browser_ev)
                                    )
                                );
                            },
                            _           => (),
                        }
                    },
                    PointerEvent::TOUCHCANCEL                           => {
                        match Self::extract_touch_from_list(&browser_ev.changed_touches(), &tracked_touch) {
                            Ok(touch)   => {
                                let dispatch = [PointerEventType::Leave];
                                if Self::handle_touch_event(
                                    &touch, &hit_area, &canvas, cnv_width, &dispatch[..], &handlers, &browser_ev
                                )
                                {
                                    tracked_touch = None;
                                }
                            },
                            _           => (),
                        }
                    },
                    PointerEvent::TOUCHEND                              => {
                        match Self::extract_touch_from_list(&browser_ev.changed_touches(), &tracked_touch) {
                            Ok(touch)   => {
                                let dispatch = [PointerEventType::Up, PointerEventType::Click, PointerEventType::Leave];
                                if Self::handle_touch_event(
                                    &touch, &hit_area, &canvas, cnv_width, &dispatch[..], &handlers, &browser_ev
                                )
                                {
                                    tracked_touch = None;
                                }
                            },
                            _           => ()
                        }
                    },
                    _                                                   => (),
                }
            }
        ) as Box<dyn FnMut(TouchEvent)>);
        let touch_relay_fn                      = touch_relay.as_ref().unchecked_ref::<Function>();
        for mev in &[   PointerEvent::TOUCHCANCEL
                    ,   PointerEvent::TOUCHEND
                    ,   PointerEvent::TOUCHMOVE
                    ,   PointerEvent::TOUCHSTART
                    ]
        {
            canvas_copy.add_event_listener_with_callback(mev, touch_relay_fn)?;
        }
        touch_relay.forget();
        Ok(())
    }

    pub fn new(canvas: HtmlCanvasElement, hit_area_: Rect<i32>) -> Result<Self, JsValue> {
        let hit_area        = Rect {
            top     : hit_area_.top     as f32,
            left    : hit_area_.left    as f32,
            bottom  : hit_area_.bottom  as f32,
            right   : hit_area_.right   as f32,
        };
        let handlers: ShEventHandlers   = Shared::new([
            Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()
        ]);
        let canvas_rc                   = Rc::new(canvas);
        Self::setup_mouse_handler(handlers.clone(), canvas_rc.clone(), hit_area.clone())?;
        let touch_supported = window()
            .map(|wnd| Object::keys(wnd.as_ref())
                .filter(&mut |val: JsValue, _idx, _arr| &val.as_string().unwrap() == "ontouchstart")
                .length()
            )
            .unwrap_or(0) > 0;
        if touch_supported {
            Self::setup_touch_handler(handlers.clone(), canvas_rc.clone(), hit_area.clone())?;
        }
        Ok(HitArea {
            handlers_by_event: handlers,
        })
    }

    pub fn add_listener<F: FnMut(&PointerEvent) + 'static>(&mut self, type_: PointerEventType, listener: F) -> &mut Self {
        self.handlers_by_event.deref().borrow_mut().index_mut(type_ as usize)
            .push(Box::new(listener) as Box::<dyn FnMut(&PointerEvent)>);
        self
    }
}
