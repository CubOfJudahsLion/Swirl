//! Module  :   Generic
//! Purpose :   Implements a generic event driver.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue 04/13/2021 03:48 PM
//! Modified:   Tue 12/14/2021 01:33 PM


use std::fmt::{self, Formatter, Debug};
use std::ops::IndexMut;

//use wasm_bindgen::prelude::*;
use js_sys::Date;
//use web_sys::console;

use super::{ActionAfterEvent, EventLike, EventEmitter};
use crate::utils::{EnumSet, EnumSetIndexed};


type Duration   = f64;  //  An amount of milliseconds.
type SystemTime = f64;  //  Like duration, measured as millseconds (since the *Unix epoch*.)


//  Specifies either a time or a number of events to sleep.
#[derive(Clone, Copy, Debug)]
enum ListenerSleep {
    NoSleep,
    TimeSleep(SystemTime, Duration),
    CountSleep(u64),
}

impl From<ActionAfterEvent> for ListenerSleep {
    fn from(action: ActionAfterEvent) -> ListenerSleep {
        use ActionAfterEvent::*;
        use ListenerSleep::*;
        match action {
            NoAction | RemoveListener   => NoSleep,
            Skip(calls)                 => CountSleep(calls),
            Sleep(duration)             => TimeSleep(Date::now(), duration as Duration),
        }
    }
}


//  A structure holding a mutable function and its partially-ordered
//  "id" value. Said id is the address of the function or closure cast
//  to usize.
struct OrderedFunction<Event>{
    id      : usize,
    sleeping: ListenerSleep,
    callback: Box<dyn FnMut(&Event) -> ActionAfterEvent>,
}

impl<E> Debug for OrderedFunction<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str(&format!("OrdereredFunction(#{:?})", self.id))
    }
}


impl<Event: EventLike> OrderedFunction<Event> {
    #[inline(always)]
    fn address_as_number<F>(f: &F) -> usize
        where F: FnMut(&Event) -> ActionAfterEvent
    {
        f as *const F as usize
    }

    fn new<F>(f: F) -> Self
        where   F: FnMut(&Event) -> ActionAfterEvent + 'static
    {
        OrderedFunction{
            id      : Self::address_as_number(&f),
            sleeping: ListenerSleep::NoSleep,
            callback: Box::new(f) as Box<dyn FnMut(&Event) -> ActionAfterEvent>,
        }
    }

    #[inline(always)]
    pub fn call(&mut self, ev: &Event) -> ActionAfterEvent {
        use ListenerSleep::*;
        let opt_call_result: Option<ActionAfterEvent> =
            match &mut self.sleeping {
                TimeSleep(start, duration)   => {
                    if Date::now() - *start >= *duration {
                        Some((self.callback)(ev))
                    }
                    else {
                        None
                    }
                },
                CountSleep(events)          => {
                    if *events == 0 {
                        Some((self.callback)(ev))
                    }
                    else {
                        *events -= 1;
                        None
                    }
                },
                NoSleep                     => {
                    Some((self.callback)(ev))
                },
            };
        match opt_call_result {
            Some(call_result)   => {
                self.sleeping = call_result.into();
                //console::log_1(&JsValue::from_str(&format!("OrderedFunction::call setting sleep state {:?}", self.sleeping)));
                call_result
            },
            None                => {
                ActionAfterEvent::NoAction
            }
        }
    }

    #[inline]
    pub fn id(&self) -> usize {
        self.id
    }
}


/// A generic event emitter type conforming to [`EventEmitter`](super::EventEmitter).
/// One may chose the type representing an event, as long as it conforms to the
/// `EventType` trait. It also constrains `EventType::EventIdType` to be an
/// `EnumSet` to take advantage of Rust's enums.
#[derive(Debug)]
pub struct GenericEmitter<Event>
    where   Event               : EventLike + 'static,
            Event::EventIdType  : EnumSet
{
    enabled         : bool,
    subscribers     : EnumSetIndexed<Event::EventIdType, Vec<OrderedFunction<Event>>>,
}

impl<Event> GenericEmitter<Event>
    where   Event               : EventLike,
            Event::EventIdType  : EnumSet
{
    const ESTIMATED_MAX_SUBSCRIBERS: usize = 8;

    pub fn new() -> Self {
        GenericEmitter {
            enabled         : true,
            subscribers     :
                EnumSetIndexed::<Event::EventIdType, Vec<OrderedFunction<Event>>>
                    ::new_mapped(
                        &mut |_: &Event::EventIdType|
                            Vec::with_capacity(Self::ESTIMATED_MAX_SUBSCRIBERS)
                    ),
        }
    }

    #[inline]
    fn binary_search(key: usize, s: &[OrderedFunction<Event>]) -> Result<usize, usize> {
        s.binary_search_by(|probe| probe.id().cmp(&key))
    }
}


impl<Event> EventEmitter<Event, usize> for GenericEmitter<Event>
    where   Event               : EventLike,
            Event::EventIdType  : EnumSet,
            usize               : From<Event::EventIdType>
{
    fn emit(&mut self, ev: &Event) {
        if self.enabled {
            let subs_len                                = self.subscribers[ev.type_()].len();
            let mut deleters: Vec<usize>                = Vec::with_capacity(subs_len);
            for idx in 0 .. subs_len {
                let listener = &mut self.subscribers[ev.type_()][idx];
                use ActionAfterEvent::*;
                if listener.call(&ev) == RemoveListener {
                    deleters.insert(0, idx);
                }
            }
            deleters
                .iter()
                .for_each(|&i| {
                    self.subscribers[ev.type_()].remove(i);
                });
        }
    }

    fn subscribe<F>(&mut self, ev_type: Event::EventIdType, listener: F) -> Option<usize>
        where   F: for<'a> FnMut(&'a Event) -> ActionAfterEvent + 'static
    {
        let ord_fn  = OrderedFunction::new(listener);
        let ord_id  = ord_fn.id();
        match GenericEmitter::<Event>::binary_search(ord_id, &self.subscribers[ev_type][..]) {
            Ok(_)   =>
                None,
            Err(i)  => {
                self.subscribers[ev_type].insert(i, ord_fn);
                Some(ord_id)
            }
        }
    }

    fn unsubscribe(&mut self, ev_type: Event::EventIdType, listener_id: usize) -> bool {
        let listeners = self.subscribers.index_mut(ev_type);
        match GenericEmitter::<Event>::binary_search(listener_id, listeners) {
            Ok(i)   => {
                listeners.remove(i);
                true
            },
            _       =>
                false
        }
    }

    #[inline(always)]
    fn enabled(&self) -> bool {
        self.enabled
    }

    #[inline(always)]
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}


#[cfg(test)]
pub mod tests {
    use std::cell::Cell;
    use std::convert::TryFrom;
    use std::rc::Rc;
    use futures::future::FutureExt;
    use futures::select;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use js_sys::{Function, Promise};
    use wasm_bindgen_test::*;
    use wasm_bindgen_futures::*;
    use crate::utils::html::get_window;
    use crate::events::*;
    use super::*;


    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum PromiseResolution {
        ByTime,
        ByEvent,
    }

    impl PromiseResolution {
        const BY_TIME_F64   : f64   = Self::ByTime  as u8 as f64;
        const BY_EVENT_F64  : f64   = Self::ByEvent as u8 as f64; 
    }

    impl From<PromiseResolution> for JsValue {
        fn from(prom_res: PromiseResolution) -> JsValue {
            match prom_res {
                PromiseResolution::ByTime  => JsValue::from(PromiseResolution::BY_TIME_F64),
                PromiseResolution::ByEvent => JsValue::from(PromiseResolution::BY_EVENT_F64),
            }
        }
    }

    impl TryFrom<JsValue> for PromiseResolution {
        type Error = JsValue;
        fn try_from(jv: JsValue) -> Result<PromiseResolution, Self::Error> {
            match jv.as_f64() {
                Some(resol) if resol == PromiseResolution::BY_TIME_F64  =>
                    Ok(PromiseResolution::ByTime),
                Some(resol) if resol == PromiseResolution::BY_EVENT_F64 =>
                    Ok(PromiseResolution::ByEvent),
                _                                                       =>
                    Err(jv),
            }
        }
    }

    /*pub fn time_limited_event_future<E, I, M>(event_type: E::EventIdType, emitter: M, limit_millis: u16) -> JsFuture
        where   E   : EventLike,
                I   : Copy, 
                M   : EventEmitter<E, I>
    {
        use crate::utils::html::get_window;
        let emitter     = Rc::new(RefCell::new(emitter));
        let prom_rc     : Rc<Cell<Option<Box<dyn FnMut(Function, Function)>>>>
                        = Rc::new(Cell::new(None));
        let mut prom_cb = |resolve: Function, _reject| {
            let resolve     = Rc::new(resolve);
            let timeout_rc  : Rc<Cell<Option<Closure<dyn FnMut()>>>>
                            = Rc::new(Cell::new(None));
            let timeout_id  : Rc<Cell<Option<i32>>>
                            = Rc::new(Cell::new(None));
            let listener_id : Rc<Cell<Option<I>>>
                            = Rc::new(Cell::new(None));
            let timeout_cb  = {
                let listener_id = sub_id.clone();
                let resolve     = resolve.clone();
                let timeout_rc  = timeout_rc.clone();
                Closure::wrap(Box::new(move || {
                    resolve.call1(&JsValue::NULL, &JsValue::from(PromiseResolution::BY_TIME_F64))
                        .expect("Couldn't invoke `resolve` callback");
                    emitter.unsubscribe(event_type, listener_id.get());
                    timeout_rc.set(None);
                }) as Box<dyn FnMut()>)
            };
            timeout_id.set(Some(get_window()
                .expect("Couldn't get browser window")
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout_cb.as_ref().unchecked_ref(),
                    limit_millis as i32
                )
                .expect("Couldn't set timeout")
            ));
            timeout_rc.set(Some(timeout_cb));
            let listener_cb     = {
                let timeout_id  = timeout_id.clone();
                let timeout_rc  = timeout_rc.clone();
                let listener_rc = listener_rc.clone();
                let resolve     = resolve.clone();
                move |_ev| {
                    get_window()
                        .expect("Couldn't get browser window")
                        .clear_timeout_with_handle(timeout_id.get());
                    timeout_rc.set(None);
                    resolve.call1(&JsValue::NULL, &JsValue::from(PromiseResolution::BY_EVENT_F64));
                    ActionAfterEvent::RemoveListener
                }
            };
            emitter.subscribe(event_type, listener_cb);
        };
        let prom    = Promise::new(&mut prom_cb);
        prom_rc.set(Some(Box::new(prom_cb) as Box<dyn FnMut(Function, Function)>));
        <JsFuture as From<Promise>>::from(prom)
    }*/

    pub fn timer_future(millis: u16) -> Result<JsFuture, JsValue> {
        let wnd         = get_window()?;
        let prom_rc     : Rc<Cell<Option<Box<dyn FnMut(Function, Function)>>>>
                        = Rc::new(Cell::new(None));
        let mut prom_cb = {
            let prom_rc = prom_rc.clone();
            move |resolve: Function, _reject: Function| {
                let timeout_rc  : Rc<Cell<Option<Closure<dyn FnMut()>>>>
                                = Rc::new(Cell::new(None));
                let timeout_cb  = {
                    let timeout_rc  = timeout_rc.clone();
                    Closure::wrap(Box::new(move || {
                        resolve
                            .call1(&JsValue::NULL, &PromiseResolution::ByTime.into())
                            .expect("Could not call `resolve` for timer_future");
                        timeout_rc.set(None);
                    }) as Box<dyn FnMut()>)
                };
                wnd.set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout_cb.as_ref().unchecked_ref(),
                    millis as i32
                ).expect("Could not set timeout for timer_future");
                timeout_rc.set(Some(timeout_cb));
                prom_rc.set(None);
            }
        };
        let res = Ok(<JsFuture as From<Promise>>::from(Promise::new(&mut prom_cb)));
        prom_rc.set(Some(Box::new(prom_cb) as Box<dyn FnMut(Function, Function)>));
        res
    }

    #[wasm_bindgen_test]
    async fn ordered_function() {
        //
        #[derive(Debug)]
        struct Something;
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct SomethingId;
        //
        impl EventLike for Something {
            type EventIdType = SomethingId;
            fn type_(&self) -> Self::EventIdType { SomethingId }
        }
        //
        fn make_ordered_function_future(
            ordfn       : Rc<RefCell<OrderedFunction<Something>>>,
            resolve_rc  : Rc<RefCell<Option<Function>>>,
            msg_rc      : Rc<Cell<Option<&'static str>>>,
            message     : &'static str
        )
            -> JsFuture
        {
            msg_rc.set(Some(message));
            <JsFuture as From<Promise>>::from({
                let prom_rc     : Rc<Cell<Option<Box<dyn FnMut(Function, Function)>>>>
                                = Rc::new(Cell::new(None));
                let mut prom_cb = {
                    let prom_rc = prom_rc.clone();
                    move |resolve: Function, _reject: Function| {
                        resolve_rc.replace(Some(resolve));
                        //let timeout_rc  : Rc<Cell<Option<Closure<dyn FnMut()>>>>
                        //                = Rc::new(Cell::new(None));
                        //let timeout_cb  = {
                        //    let timeout_rc  = timeout_rc.clone();
                        //    let ordfn       = ordfn.clone();
                        //    Closure::wrap(Box::new(move || {
                              ordfn.borrow_mut().call(&Something);
                        //        timeout_rc.set(None);
                        //    }) as Box<dyn FnMut()>)
                        //};
                        //get_window()
                        //    .unwrap()
                        //    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        //        timeout_cb.as_ref().unchecked_ref::<Function>(),
                        //        0
                        //    )
                        //    .expect("Couldn't set timer for make_ordered_function_future");
                        //timeout_rc.replace(Some(timeout_cb));
                        prom_rc.set(None);
                    }
                };
                let prom    = Promise::new(&mut prom_cb);
                prom_rc.set(Some(Box::new(prom_cb) as Box<dyn FnMut(Function, Function)>));
                prom
            })
        }
        //
        let msg_rc      : Rc<Cell<Option<&'static str>>>
                        = Rc::new(Cell::new(None));
        let ordfn_res   = Rc::new(Cell::new(ActionAfterEvent::NoAction));
        let resolve_rc  : Rc<RefCell<Option<Function>>>
                        = Rc::new(RefCell::new(None));
        let ordfn       = Rc::new(RefCell::new({
            let ordfn_res   = ordfn_res.clone();
            let resolve_rc  = resolve_rc.clone();
            let msg_rc      = msg_rc.clone();
            OrderedFunction::new(move |_: &Something| {
                if let Some(ref func) = *resolve_rc.borrow() {
                    func
                        .call1(&JsValue::NULL, &PromiseResolution::ByEvent.into())
                        .expect("Couldn't call ordered function");
                }
                if let Some(strref) = msg_rc.get() {
                    console_log!("{}", strref);
                }
                ordfn_res.get()
            })
        }));
        //  Here we call it twice just to prove that a consecutive
        //  call after the previous returned `NoAction` succeeds.
        assert!(
            make_ordered_function_future(
                ordfn.clone(),
                resolve_rc.clone(),
                msg_rc.clone(),
                "first call"
            ).await.is_ok()
        );
        assert!(
            make_ordered_function_future(
                ordfn.clone(),
                resolve_rc.clone(),
                msg_rc.clone(),
                "second call"
            ).await.is_ok()
        );
        //  Now, we skip one call.
        ordfn_res.set(ActionAfterEvent::Skip(1));
        //  This call will run. The next one won't.
        assert!(
            make_ordered_function_future(
                ordfn.clone(),
                resolve_rc.clone(),
                msg_rc.clone(),
                "setting to Skip(1)"
            ).await.is_ok()
        );
        console_log!("Set to skip.");
        //ordfn_res.set(ActionAfterEvent::NoAction);
        let skip_result = select!{
            timer   =   timer_future(2).unwrap().fuse() => timer,
            event   =   make_ordered_function_future(
                            ordfn.clone(),
                            resolve_rc.clone(),
                            msg_rc.clone(),
                            "this call is under a Skip(1); \
                            shouldn't be called"
                        ).fuse()                        => event,
        };
        console_log!("Right after skipped call.");
        //  This should have been resolved by the timer.
        assert_eq!(
            skip_result
                .and_then(|skip_result|
                    <PromiseResolution as TryFrom<JsValue>>::try_from(skip_result)
                ),
            Ok(PromiseResolution::ByTime)
        );
        //  The following call should run again. We'll also use it to set a timed sleep.
        ordfn_res.set(ActionAfterEvent::Sleep(5));
        assert!(
            make_ordered_function_future(
                ordfn.clone(),
                resolve_rc.clone(),
                msg_rc.clone(),
                "Call after Skip(1). You should see this one. Also, setting 5ms sleep."
            ).await.is_ok()
        );
        //  Check that sleep is actually on.
        assert!(
            if let &ListenerSleep::TimeSleep(_, duration) = &ordfn.borrow().sleeping {
                duration == 5.
            }
            else {
                false
            },
            "Duration set with wrong parameter"
        );
        //  Later calls should not request sleep.
        console_log!("Setting return to NoAction");
        ordfn_res.set(ActionAfterEvent::NoAction);
        //  Record time
        let mark = Date::now();
        let pre_sleep_mark = mark;
        console_log!("Right before timed sleep call. Mark.");
        //  Now a race to see who responds first. 4 millisecond timer,
        //  or the function that's been inhibited for five milliseconds.
        let timed_sleep_result = select!{
            timer   =   timer_future(3).unwrap().fuse() => timer,
            event   =   make_ordered_function_future(
                            ordfn.clone(),
                            resolve_rc.clone(),
                            msg_rc.clone(),
                            "Under a Sleep(5). \
                            You shouldn't see this one."
                        ).fuse()                        => event,
        };
        console_log!("Right after select! with timed sleep call, millis after mark: {}", Date::now() - mark);
        //  This, too, should have been resolved by the timer.
        assert_eq!(
            timed_sleep_result
                .and_then(|result|
                    <PromiseResolution as TryFrom<JsValue>>::try_from(result)
                ),
            Ok(PromiseResolution::ByTime)
        );
        //  Test that at least 4ms passed
        assert!(Date::now() - mark >= 3., "Less than 3ms passed");
        //  Check that the sleep timer is still on.
        assert!(
            match &ordfn.borrow().sleeping {
                &ListenerSleep::TimeSleep(_, _) => true,
                _                               => false,
            },
            "Sleep record disappeared after call that 'didn't happen'."
        );
        console_log!("Marking before extra 3ms wait.");
        let mark = Date::now();
        let two_secs = timer_future(3)
            .expect("Couldn't set two-second wait");
        let two_secs_await = two_secs.await;
        assert!(Date::now() - mark >= 3., "Less than 3ms passed");
        console_log!("Right after wait. Time after mark: {}ms.", Date::now() - mark);
        console_log!("Time after first mark: {}ms.", Date::now() - pre_sleep_mark);
        assert!(two_secs_await.is_ok());
        //  Brief test for our own `timer_future` function.
        //  Check that the sleep record is still there.
        assert!(
            match &ordfn.borrow().sleeping {
                &ListenerSleep::TimeSleep(_, _) => true,
                _                               => false,
            },
            "Sleep record disappeared after 6ms+ wait"
        );
        //  It's been 6 millis now. Timed sleep should have run out.
        assert!(
            if let &ListenerSleep::TimeSleep(start, duration) = &ordfn.borrow().sleeping {
                console_log!("Now: {}, Start: {}, Duration: {}", Date::now(), start, duration);
                Date::now() - start >= duration
            }
            else {
                console_log!("Sleep record gone after 6ms wait");
                false
            },
            "Measure error in elapsed time"
        );
        //  And the function should be callable again.
        assert!(
            make_ordered_function_future(
                ordfn.clone(),
                resolve_rc.clone(),
                msg_rc.clone(),
                "Last call (you should see it)"
            ).await.is_ok()
        ); 
    }
}
