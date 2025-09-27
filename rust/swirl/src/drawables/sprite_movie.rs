//! Module  :   Sprite Movie
//! Purpose :   Creates an imageb
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Mon 10/04/2021 03:30 PM
//! Modified:   Wed 05/04/2022 11:21 PM


use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};

use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_val, JsCast};
use web_sys::{window, CanvasRenderingContext2d, HtmlImageElement};

use crate::math::geom::rect::*;
use crate::base::bounded::Bounded;
use crate::base::drawable::*;

use crate::utils::shared::Shared;
use crate::utils::enum_set::*;
use crate::events::*;


make_enum_set!{
    SpriteMovieEventType ::=
        Start       <=> "Start",
        Stop        <=> "Stop",
        //Pause       <=> "Pause",
        //Resume      <=> "Resume",
        Loop        <=> "Loop",
        EnterFrame  <=> "EnterFrame",
        ExitFrame   <=> "ExitFrame",
}

/// events issued by the `SpriteMovie`.
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpriteMovieEvent {
    type_   : SpriteMovieEventType,
    frame   : usize,
}

impl SpriteMovieEvent {
    #[inline(always)]
    pub fn frame(&self) -> usize { self.frame }
}

impl EventLike for SpriteMovieEvent {
    type EventIdType = SpriteMovieEventType;
    fn type_(&self) -> Self::EventIdType { self.type_ }
}


/// A frame event emitter.
pub type SpriteMovieEventEmitter = GenericEmitter<SpriteMovieEvent>;


// //  A sprite movie has a default user-specified wait before
// //  the next frame shows, but any given frame can have
// //  a particular duration. Also, to save bandwidth, it can
// //  start 
// #[derive(Clone, Debug)]
// pub enum MovieFrameSpec<Img> {
//     //  Im from Image alone.
//     Im(Img),
//     //  Image timing
//     ImT(Img, u16),
//     //  Image with offset (i, x,y)
//     ImO(Img, i16, i16),
//     //  Image with both timing and offset (i, duration, x, y)
//     ImTO(Img, u16, i16, i16)
// }
// 
// impl<I> MovieFrameSpec<I> {
//     pub fn image_ref(&self) -> &I {
//         use MovieFrameSpec::*;
//         match self {
//             &Im(ref img)            => img,
//             &ImT(ref img, _)        => img,
//             &ImO(ref img, _, _)     => img,
//             &ImTO(ref img, _, _, _) => img,
//         }
//     }
// 
//     pub fn full_spec(&self, default_timing: u16) -> (&I, u16, i16, i16) {
//         use MovieFrameSpec::*;
//         match self {
//             &Im(ref img)                    => (img, default_timing, 0, 0),
//             &ImT(ref img, timing)           => (img, timing, 0, 0),
//             &ImO(ref img, x, y)             => (img, default_timing, x, y),
//             &ImTO(ref img, timing, x, y)    => (img, timing, x, y),
//         }
//     }
// }
// 
// /// Specialization for specifying frames for the movie.
// pub type MovieFrame = MovieFrameSpec<HtmlImageElement>;
// 
// 
// //  Specifies the way a movie will play.
// #[derive(Clone, Copy, Debug)]
// pub enum PlaybackReps {
//     //  Play non-stop (unless user stops.)
//     Indefinite,
//     //  Stop at some specified frame, after
//     //  a number of repetitions. (Zero means
//     //  playing only until the frame shows
//     //  for its first and only time.)
//     Times(u16),
// }
// 
// 
// //  Specifies the range of frames to play. We can
// //  play the entire movie, or just a subsection of it.
// #[derive(Clone, Debug)]
// pub enum PlaybackRange {
//     All,
//     Restricted {
//         start   : usize,
//         end     : usize,
//     },
// }
// 
// 
// //  Direction of play
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum PlaybackDirection {
//     PlayForward,
//     PlayBackward,
// }
// 
// 
// //  Complete playback specification
// struct PlaybackPlan {
//     timeout_id      : Option<i32>,
//         //  ^ Optional because we can pause --
//         //  we keep the plan even if we don't
//         //  have a timeout.
//     direction       : PlaybackDirection,
//     reps            : PlaybackReps,
//     range           : PlaybackRange,
// }


//  The inner structure. It carries `Drawable` attributes,
//  bounds, frame data, playback parameters and an event emitter.
struct SpriteMovieInner {
    //  Drawable
    visible         : bool,
    alpha           : f64,
    blend_mode      : BlendMode,
    //  Bounded
    max_frame_dim   : (i16, i16),
    //  Frame and playback-related
    frame_duration  : u16,
    //cyclic          : bool,
    //frames          : Box<[MovieFrame]>,
    frames          : Box<[HtmlImageElement]>,
    current_frame   : usize,
    //plan            : Option<PlaybackPlan>,
    timeout         : Option<(i32, Closure<dyn FnMut()>)>,
    //  Events
    emitter         : SpriteMovieEventEmitter,
}

impl Debug for SpriteMovieInner {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let timeout_str =
            match &self.timeout {
                &None           => "None".to_string(),
                &Some((id, _))  => format!("Some(({:?},...))", id),
            };
        f.debug_struct("SpriteMovieInner")
            .field("visible", &self.visible)
            .field("alpha", &self.alpha)
            .field("blend_mode", &self.blend_mode)
            .field("max_frame_dim", &self.max_frame_dim)
            .field("frame_duration", &self.frame_duration)
            .field("frames", &self.frames)
            .field("current_frame", &self.current_frame)
            .field("timeout", &timeout_str)
            .field("emitter", &self.emitter)
            .finish()
    }
}


/// A 'movie' in the most classic sense: a series of
/// images that play in succession -- in this case,
/// a list of `HtmlImageElement`s.
#[derive(Clone, Debug)]
pub struct SpriteMovie(Shared<SpriteMovieInner>);


/// Types of errors that can be returned by improperly
/// constructed function calls.
#[derive(Clone, Debug)]
pub enum SpriteMovieError {
    NoFrames,
    TooManyFrames,
    FrameIndexError,
    DuplicateTimings,
    SystemFailure(JsValue),
}

impl From<SpriteMovieError> for JsValue {
    fn from(jsval: SpriteMovieError) -> Self {
        use SpriteMovieError::*;
        match jsval {
            NoFrames                    => JsValue::from_str("No frames"),
            TooManyFrames               => JsValue::from_str("Too many frames"),
            FrameIndexError             => JsValue::from_str("Frame indexing error"),
            DuplicateTimings            => JsValue::from_str("Duplicate timings"),
            SystemFailure(inner_js_val) => inner_js_val,
        }
    }
}


impl Bounded<i16> for SpriteMovie {
    fn bounds(&self) -> Option<Rect<i16>> {
        Some(Rect::new(0, 0, self.0.max_frame_dim.1 - 1, self.0.max_frame_dim.0 - 1))
    }
}


impl Drawable for SpriteMovie {
    fn drawable(&self) -> bool { self.0.visible }
    fn set_drawable(&mut self, drawable: bool) { self.0.visible = drawable; }

    fn blend_mode(&self) -> BlendMode { self.0.blend_mode }
    fn set_blend_mode(&mut self, blend_mode_p: BlendMode) {
        self.0.blend_mode = blend_mode_p;
    }

    fn alpha(&self) -> f64 { self.0.alpha }
    fn set_alpha(&mut self, alpha_p: f64) {
        self.0.alpha =
            if alpha_p < 0. {
                0.
            }
            else if alpha_p > 1. {
                1.
            }
            else {
                alpha_p
            };
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let idx             = self.0.current_frame;
        //let frame_spec      = &self.0.frames[idx];
        // let (img, _, x, y)  = frame_spec.full_spec(self.0.frame_duration);
        // ctx.draw_image_with_html_image_element(img, x as f64, y as f64)
        ctx.draw_image_with_html_image_element(&self.0.frames[idx], 0., 0.)
    }
}


impl SpriteMovie {
    /// Create a `SpriteMovie` with default duration and
    /// per-frame if needed. Do note that a `SpriteMovie` is
    /// restricted to a maximum of 65536 frames (i.e., what
    /// fits in a `u16`.) For longer movies, (or, in a more
    /// practical light bandwidth-wise, anything going close
    /// to a hundred frames), video should be considered
    /// instead.
    pub fn new(
        default_duration: u16,
        frames          : &[HtmlImageElement]
        //frames          : &[MovieFrame]
    )
        -> Result<Self, SpriteMovieError>
    {
        let frame_count = frames.len();
        if frame_count == 0 {
            Err(SpriteMovieError::NoFrames)
        }
        else if frame_count > u16::MAX as usize {
            Err(SpriteMovieError::TooManyFrames)
        }
        else {
            let max_frame_dim       = frames
                .iter()
                //.map(|spec| spec.full_spec(default_duration))
                .fold(
                    (i16::MIN, i16::MIN),
                    // |(max_width, max_height), (img, _, x_ofs, y_ofs)|
                    |(max_width, max_height), img| {
                        //let width   = img.width()  as i16 + x_ofs;
                        //let height  = img.height() as i16 + y_ofs;
                        let width   = img.width()  as i16;
                        let height  = img.height() as i16;
                        (   if width  > max_width  { width }  else { max_width }
                        ,   if height > max_height { height } else { max_height }
                        )
                    }
                );
            Ok(SpriteMovie(Shared::new(SpriteMovieInner {
                visible         : true,
                alpha           : 1.,
                blend_mode      : BlendMode::Normal,
                max_frame_dim,
                frame_duration  : default_duration,
                //cyclic          : false,
                frames          : frames.to_vec().into_boxed_slice(),
                current_frame   : 0,
                //plan            : None,
                timeout         : None,
                emitter         : GenericEmitter::<SpriteMovieEvent>::new(),
            })))
        }
    }

    //pub fn new_simple(frame_duration: u16, images: &[HtmlImageElement]) -> Result<Self, SpriteMovieError> {
    //    Self::new(
    //        frame_duration,
    //        &images
    //            .iter()
    //            .map(|i| MovieFrameSpec::Im(i.clone()))
    //            .collect::<Vec<_>>(),
    //        &[]
    //    )
    //}

    #[inline(always)]
    pub fn frame_count(&self) -> usize { self.0.frames.len() }

    #[inline(always)]
    pub fn current_frame(&self) -> usize { self.0.current_frame }

    #[inline(always)]
    pub fn is_playing(&self) -> bool {
        //self.0.plan
        //    .and_then(|plan| plan.frame_timeout.is_some())
        //    .unwrap_or(false)
        self.0.timeout.is_some()
    }

    pub fn stop(&mut self) -> Result<bool, JsValue> {
        if let Some((id, _)) = self.0.timeout {
            window()
                .ok_or_else(|| JsValue::from_str("SpriteMovie::stop(): couldn't find main window"))?
                .clear_interval_with_handle(id);
            self.0.timeout      = None;
            let current_frame   = self.current_frame();
            self.0.emitter.emit(&SpriteMovieEvent {
                type_   : SpriteMovieEventType::Stop,
                frame   : current_frame,
            });
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    fn change_frame(
        &mut self,
        from                : usize,
        to                  : usize,
        pre_change_event    : Option<SpriteMovieEventType>,
        post_change_event   : Option<SpriteMovieEventType>
    )
    {
        if from != to {
            self.0.emitter.emit(&SpriteMovieEvent {
                type_   : SpriteMovieEventType::ExitFrame,
                frame   : from,
            });
            if let Some(type_) = pre_change_event {
                self.0.emitter.emit(&SpriteMovieEvent {
                    type_,
                    frame   : from,
                });
            }
            self.0.current_frame = to;
            if let Some(type_) = post_change_event {
                self.0.emitter.emit(&SpriteMovieEvent {
                    type_,
                    frame   : to,
                });
            }
            self.0.emitter.emit(&SpriteMovieEvent {
                type_   : SpriteMovieEventType::EnterFrame,
                frame   : to,
            });
        }
    }

    pub fn play(&mut self, cyclic: bool) -> Result<(), JsValue> {
        self.stop()?;
        let frame_at_start  = self.current_frame();
        if frame_at_start != 0 {
            self.change_frame(frame_at_start, 0, None, None);
        }
        self.0.emitter.emit(&SpriteMovieEvent {
            type_   : SpriteMovieEventType::Start,
            frame   : 0,
        });
        let anim            = {
            let mut other_self = self.clone();
            Closure::wrap(Box::new(move || {
                let curr_frame = other_self.current_frame();
                if curr_frame + 1 >= other_self.frame_count() {
                    if cyclic {
                        other_self.change_frame(
                            curr_frame,
                            0,
                            Some(SpriteMovieEventType::Loop),
                            None
                        );
                    }
                    else {
                        if let Err(js_val) = other_self.stop() {
                            throw_val(js_val);
                        }
                    }
                }
                else {
                    other_self.change_frame(curr_frame, curr_frame + 1, None, None);
                }
            }) as Box<dyn FnMut()>)
        };
        let interval_id = window()
            .ok_or_else(|| JsValue::from_str("SpriteMovie::play(): couldn't find main window"))?
            .set_interval_with_callback_and_timeout_and_arguments_0(
                anim.as_ref().unchecked_ref(),
                self.0.frame_duration as i32
            )?;
        self.0.timeout = Some((interval_id, anim));
        Ok(())
    }

    // pub fn is_paused(&self) -> bool {
    //     self.0.plan
    //         .and_then(|plan| plan.frame_timeout.is_none())
    //         .unwrap_or(false)
    // }

    // fn pause(&mut self) -> Result<bool, JsValue> {
    //     let result =
    //         if let Some(ref mut plan) = self.0.plan {
    //             if let Some(id) = plan.timeout_id {
    //                 if let Some(wnd) = window() {
    //                     wnd.clear_timeout_with_handle(id);
    //                     plan.timeout_id = None;
    //                     Ok(true)
    //                 }
    //                 else {
    //                     Err(JsValue::from_str("Couldn't find main window"))
    //                 }
    //             }
    //             else {
    //                 Ok(false)
    //             }
    //         }
    //         else {
    //             Ok(false)
    //         };
    //     if let &Ok(true) = &result {
    //         self.0.emmiter.emit(SpriteMovieEvent {
    //             type_   : SpriteMovieEventType::Paused,
    //             frame   : self.0.current_frame,
    //         });
    //     }
    //     result
    // }

    // fn message_and_stop(&mut self, op_ev: Option<SpriteMovieEvent>) -> Result<bool, JsValue> {
    //     let result =
    //         if let Some(ref mut plan) = self.0.plan {
    //             if let Some(wnd) = window() {
    //                 wnd.clear_timeout_with_handle(plan.timeout_id);
    //                 *plan = None;
    //                 Ok(true)
    //             }
    //             else {
    //                 Err(JsValue::from_str("Couldn't find main window"))
    //             }
    //         }
    //         else {
    //             Ok(false)
    //         };
    //     if let &Ok(true) = &result {
    //         if let Some(ev) = op_ev {
    //             self.0.emmiter.emit(ev);
    //         }
    //         self.0.emitter.emit(SpriteMovieEvent {
    //             type_   : SpriteMovieEventType::Stopped,
    //             frame   : self.0.current_frame,
    //         });
    //     }
    //     result
    // }

    // #[inline(always)]
    // pub fn stop(&mut self) -> Result<bool, JsValue> {
    //     self.message_and_stop(None)
    // }

    // pub fn goto_and_stop(&mut self, frame: usize) -> Result<bool, SpriteMovieError> {
    //     if frame >= self.0.frames.len() {
    //         Err(SpriteMovieError::FrameIndexError)
    //     }
    //     else {
    //         let op_msg =
    //             if frame != self.0.current_frame {
    //                 Some(SpriteMovieEvent {
    //                     type_   : SpriteMovieEventType::EnteredFrame,
    //                     frame,
    //                 })
    //             }
    //             else {
    //                 None
    //             };
    //         let was_stopped         = self
    //             .message_and_stop(op_msg)
    //             .map_err(|err| SpriteMovieError::SystemFailure(err))?;
    //         self.0.current_frame    = frame;
    //         Ok(was_stopped)
    //     }
    // }

    // fn play_full_spec(
    //     &mut self,
    //     start_frame : usize,
    //     end_frame   : Option<usize>,
    //     direction   : PlaybackDirection,
    //     reps        : PlaybackReps,
    // )
    //     -> Result<(), SpriteMovieError>
    // {
    //     let frame_count = self.frame_count();
    //     if start_frame >= frame_count || end_frame >= frame_count {
    //         
    //     }
    // }
}


impl Deref for SpriteMovie {
    type Target = SpriteMovieEventEmitter;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0.emitter }
}

impl DerefMut for SpriteMovie {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0.emitter }
}


#[cfg(test)]
pub mod tests {
    use futures::join;
    use std::rc::Rc;
    use std::cell::Cell;
    use wasm_bindgen_test::*;
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen::JsCast;
    use js_sys::{Function, JsString, Promise};
    //use web_sys::console;
    use super::*;

    pub fn make_dummy_image() -> Result<HtmlImageElement, JsValue> {
        let img = window()
            .ok_or_else(|| JsValue::from_str("Couldn't find main Window."))?
            .document()
            .ok_or_else(|| JsValue::from_str("Couldn't find Document element."))?
            .create_element("img")?
            .dyn_into::<HtmlImageElement>()
            .map_err(|_| JsValue::from_str("Failed to create <img> as proper HTMLImageElement."))?;
        img
            .set_src(
                "data:image/jpeg;base64,\
                /9j/4AAQSkZJRgABAQEASABIAAD/2wBDAAMCAgMCAgMDAwMEAwMEBQgFBQQEBQoHBwYIDAoMDAsK\
                CwsNDhIQDQ4RDgsLEBYQERMUFRUVDA8XGBYUGBIUFRT/2wBDAQMEBAUEBQkFBQkUDQsNFBQUFBQU\
                FBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBT/wAARCAAEAAQDASIA\
                AhEBAxEB/8QAFQABAQAAAAAAAAAAAAAAAAAAAAj/xAAdEAABAwUBAAAAAAAAAAAAAAAAAQhRBQYX\
                JUFh/8QAFAEBAAAAAAAAAAAAAAAAAAAAAP/EABQRAQAAAAAAAAAAAAAAAAAAAAD/2gAMAwEAAhED\
                EQA/AKybW22w8S0rVT1ITwAAf//Z"
            );
        Ok(img)
    }

    pub fn null_future_within(millis: u16) -> JsFuture {
        JsFuture::from(
            Promise::new(&mut |success, failure| {
                if let Err(jval) = window()
                    .ok_or_else(|| JsValue::from_str("empty_future_within(): failed to locate main Window."))
                    .and_then(|win| {
                        let callback = Closure::wrap(Box::new(move || {
                            let _ = success.call1(&JsValue::NULL, &JsValue::NULL);
                        }) as Box<dyn FnMut()>);
                        let result = win
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                callback.as_ref().unchecked_ref(),
                                millis as i32
                            );
                        callback.forget();
                        result
                    })
                {
                    let _ = failure.call1(&JsValue::NULL, &jval);
                }
            })
        )
    }

    #[wasm_bindgen_test]
    pub async fn playing_and_looping() {
        match make_dummy_image() {
            Ok(img) => {
                if let Ok(mut smov) = SpriteMovie::new(40, &[img.clone(), img.clone(), img]) {
                    assert!(smov.play(false).is_ok());
                    assert!(smov.is_playing());
                    assert_eq!(smov.current_frame(), 0);
                    let _ = null_future_within(41).await;
                    assert!(smov.is_playing());
                    assert_eq!(smov.current_frame(), 1);
                    let _ = null_future_within(40).await;
                    assert!(smov.is_playing());
                    assert_eq!(smov.current_frame(), 2);
                    let _ = null_future_within(40).await;
                    assert!(!smov.is_playing());
                    assert_eq!(smov.current_frame(), 2);
                    assert!(smov.play(true).is_ok());
                    let _ = null_future_within(161).await;
                    assert_eq!(smov.current_frame(), 1);
                    assert!(smov.is_playing());
                    assert!(smov.stop().is_ok());
                    assert!(!smov.is_playing());
                }
                else {
                    panic!("SpriteMovieError triggered.");
                }
            },
            Err(js) => {
                panic!("{}", String::from(JsString::from(js)));
            }
        }
    }


    pub fn event_as_future(mut smov: SpriteMovie, event: SpriteMovieEventType, wait_for_frame: Option<usize>) -> JsFuture {
        //
        let mut promise_resolve = move |resolve: Function, _reject: Function| {
            //console::log_1(&JsValue::from_str(&format!("Length of success function: {}.", resolve.length())));
            let sub_id = Rc::new(Cell::new(usize::MAX));
            {
                let sub_id_copy     = sub_id.clone();
                let mut smov_copy   = smov.clone();
                sub_id.set(smov.subscribe(event, move |ev| {
                    if wait_for_frame.unwrap_or(ev.frame) == ev.frame {
                        //console::log_1(&JsValue::from_str("Event subscriber called."));
                        if sub_id_copy.get() == usize::MAX {
                            panic!("Unaltered subscriber id.");
                        }
                        assert!(resolve.call1(&JsValue::null(), &JsValue::from_f64(ev.frame as f64)).is_ok());
                        assert!(smov_copy.unsubscribe(event, sub_id_copy.get()));
                    }
                    ActionAfterEvent::NoAction
                }).unwrap());
            }
        };
        //
        JsFuture::from(Promise::new(&mut promise_resolve as &mut dyn FnMut(Function, Function)))
    }


    #[wasm_bindgen_test]
    pub async fn message_sequence() {
        match (window(), make_dummy_image()) {
            (Some(wnd), Ok(img))    => {
                if let Ok(smov) = SpriteMovie::new(20, &[img.clone(), img.clone(), img]) {
                    //  First, let's try with an auto-stopping movie. All events.
                    use SpriteMovieEventType::*;
                    let closure = {
                        let mut smov = smov.clone();
                        Closure::wrap(Box::new(move || {
                            assert!(smov.play(false).is_ok());
                        }) as Box::<dyn FnMut()>)
                    };
                    assert!(wnd.set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        0
                    ).is_ok());
                    closure.forget();
                    let first       = event_as_future(smov.clone(), Start, None).await.unwrap().as_f64().unwrap() as usize;
                    assert_eq!(first, 0);
                    //console::log_1(&JsValue::from_str("Start, frame 0."));
                    let next = join!(
                        event_as_future(smov.clone(), ExitFrame, None),
                        event_as_future(smov.clone(), EnterFrame, None)
                    );
                    assert_eq!(next.0.unwrap().as_f64().unwrap(), 0.);
                    //console::log_1(&JsValue::from_str("ExitFrame, frame 0."));
                    assert_eq!(next.1.unwrap().as_f64().unwrap(), 1.);
                    //console::log_1(&JsValue::from_str("EnterFrame, frame 1."));
                    let next = join!(
                        event_as_future(smov.clone(), ExitFrame, None),
                        event_as_future(smov.clone(), EnterFrame, None),
                        event_as_future(smov.clone(), Stop, None)
                    );
                    assert_eq!(next.0.unwrap().as_f64().unwrap(), 1.);
                    //console::log_1(&JsValue::from_str("ExitFrame, frame 1."));
                    assert_eq!(next.1.unwrap().as_f64().unwrap(), 2.);
                    //console::log_1(&JsValue::from_str("EnterFrame, frame 2."));
                    assert_eq!(next.2.unwrap().as_f64().unwrap(), 2.);
                    //console::log_1(&JsValue::from_str("Stop, frame 2."));
                    assert!(!smov.is_playing());
                    //  Now a cycling movie.
                    let closure = {
                        let mut smov = smov.clone();
                        Closure::wrap(Box::new(move || {
                            assert!(smov.play(true).is_ok());
                        }) as Box::<dyn FnMut()>)
                    };
                    assert!(wnd.set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        0
                    ).is_ok());
                    closure.forget();
                    let next = join!(
                        event_as_future(smov.clone(), EnterFrame, None),
                        event_as_future(smov.clone(), Start, None),
                    );
                    assert_eq!(next.1.unwrap().as_f64().unwrap(), 0.);
                    //console::log_1(&JsValue::from_str("EnterFrame, frame 0."));
                    assert_eq!(next.0.unwrap().as_f64().unwrap(), 0.);
                    //console::log_1(&JsValue::from_str("Start, frame 0."));
                    let next = join!(
                        event_as_future(smov.clone(), ExitFrame, Some(2)),
                        event_as_future(smov.clone(), Loop, None),
                        event_as_future(smov.clone(), EnterFrame, Some(0))
                    );
                    assert_eq!(next.0.unwrap().as_f64().unwrap(), 2.);
                    //console::log_1(&JsValue::from_str("... ExitFrame, frame 2."));
                    assert_eq!(next.1.unwrap().as_f64().unwrap(), 2.);
                    //console::log_1(&JsValue::from_str("Loop, frame 2."));
                    assert_eq!(next.2.unwrap().as_f64().unwrap(), 0.);
                    //console::log_1(&JsValue::from_str("EnterFrame, frame 0."));
                    assert!(smov.is_playing());
                    let closure     = {
                        let mut smov    = smov.clone();
                        Closure::wrap(Box::new(move || {
                            assert!(smov.stop().is_ok());
                        }) as Box::<dyn FnMut()>)
                    };
                    assert!(wnd.set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        0
                    ).is_ok());
                    closure.forget();
                    assert_eq!(event_as_future(smov.clone(), Stop, None).await.unwrap().as_f64().unwrap(), 0.);
                    assert!(!smov.is_playing());
                }
                else {
                    panic!("SpriteMovieError triggered.");
                }
            },
            _                       => {
                panic!("Resource access/creation failure");
            },
        }
    }
}
