//! Module  :   Sound Instance
//! Purpose :   An instance of an audio clip, where its "*class*" or
//!             "*template*" is an `HtmlAudioElement`. It has volume,
//!             panning, play, pause, loop and rewind capabilities,
//!             and it can be subscribed to for event listening.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Wed 11/24/2021 10:30 AM
//! Modified:   Fri 12/17/2021 12:26 PM


use std::cell::{Cell, RefCell};
use std::ops::Drop;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Promise;
use web_sys::{
    //console,
    HtmlAudioElement,
    MediaElementAudioSourceNode, GainNode, StereoPannerNode
};

use crate::events::*;
use crate::events::generic::GenericEmitter;
use super::SoundEvent;


#[derive(Clone, Debug)]
pub struct SoundInstance {
    audio_tag   : HtmlAudioElement,
    _audio_node : MediaElementAudioSourceNode,
    gain_node   : GainNode,
    pan_node    : StereoPannerNode,
    emitter     : Rc<RefCell<GenericEmitter<SoundEvent>>>,
    _ev_drivers : Rc<[Closure<dyn FnMut()>; 4]>,
}

impl Drop for SoundInstance {
    fn drop(&mut self) {
        self.audio_tag.set_onpause(None);
        let _ = self.audio_tag.pause();
        let _ = self.gain_node.disconnect();
        self.audio_tag.set_onplay(None);
        self.audio_tag.set_onended(None);
        self.audio_tag.set_onseeked(None);
    }
}

impl SoundInstance {
    //  Default initial volume: full.
    const INITIAL_VOL: f32 = 1.;
    //  Default initial panning: centered.
    const INITIAL_PAN: f32 = 0.;

    //  Private constructor.  With the exception of the `audio_tag`,
    //  its elements are constructed by a browser's `AudioContext`,
    //  so it makes sense to make it a client class of `SoundContext`.
    pub(super) fn new(
        audio_tag   : HtmlAudioElement,
        audio_node  : MediaElementAudioSourceNode,
        gain_node   : GainNode,
        pan_node    : StereoPannerNode,
        dest        : &GainNode
    )
        -> Result<Self, JsValue>
    {
        pan_node.pan().set_value(Self::INITIAL_PAN);
        gain_node.gain().set_value(Self::INITIAL_VOL);
        gain_node.connect_with_audio_node(dest)?;
        pan_node.connect_with_audio_node(&gain_node)?;
        audio_node.connect_with_audio_node(&pan_node)?;
        let emitter     = Rc::new(RefCell::new(GenericEmitter::<SoundEvent>::new()));
        let onpause_cb  = {
            let emitter = emitter.clone();
            Closure::wrap(Box::new(move || {
                emitter.borrow_mut().emit(&SoundEvent::Paused);
            }) as Box<dyn FnMut()>)
        };
        let onplay_cb   = {
            let emitter = emitter.clone();
            Closure::wrap(Box::new(move || {
                emitter.borrow_mut().emit(&SoundEvent::Playing);
            }) as Box<dyn FnMut()>)
        };
        let onended_cb  = {
            let emitter = emitter.clone();
            Closure::wrap(Box::new(move || {
                //console::log_1(&JsValue::from_str("Inside onended_cb"));
                emitter.borrow_mut().emit(&SoundEvent::Ended);
            }) as Box<dyn FnMut()>)
        };
        let onseeked_cb = {
            let emitter = emitter.clone();
            Closure::wrap(Box::new(move || {
                emitter.borrow_mut().emit(&SoundEvent::Seeked);
            }) as Box<dyn FnMut()>)
        };
        audio_tag.set_onpause(Some(onpause_cb.as_ref().unchecked_ref()));
        audio_tag.set_onplay(Some(onplay_cb.as_ref().unchecked_ref()));
        audio_tag.set_onended(Some(onended_cb.as_ref().unchecked_ref()));
        audio_tag.set_onseeked(Some(onseeked_cb.as_ref().unchecked_ref()));
        Ok(SoundInstance {
            audio_tag,
            _audio_node : audio_node,
            gain_node,
            pan_node,
            emitter,
            _ev_drivers : Rc::new([onpause_cb, onplay_cb, onended_cb, onseeked_cb]),
        })
    }

    #[inline(always)]
    pub fn duration(&self) -> f64 { self.audio_tag.duration() }

    #[inline(always)]
    pub fn current_time(&self) -> f64 { self.audio_tag.current_time() }

    #[inline(always)]
    pub fn set_current_time(&mut self, time: f64) { self.audio_tag.set_current_time(time); }

    #[inline(always)]
    pub fn looping(&self) -> bool { self.audio_tag.loop_() }

    #[inline(always)]
    pub fn set_looping(&mut self, loop_: bool) { self.audio_tag.set_loop(loop_); }

    #[inline(always)]
    pub fn play(&mut self) -> Result<Promise, JsValue> { self.audio_tag.play() }

    #[inline(always)]
    pub fn pause(&mut self) -> Result<(), JsValue> { self.audio_tag.pause() }

    pub fn stop_and_rewind(&mut self) -> Result<(), JsValue> {
        let seek_ranges = self.audio_tag.seekable();
        if seek_ranges.length() == 0 {
            Err(JsValue::from_str("SoundInstance::stop_and_rewind: media is not seekable"))
        }
        else {
            let start_time = seek_ranges.start(0)?;
            if start_time == self.audio_tag.current_time() {
                self.audio_tag.pause()?;
                self.emitter.borrow_mut().emit(&SoundEvent::Rewinded);
                Ok(())
            }
            else {
                self.audio_tag.pause()?;
                let cb_rc   : Rc<Cell<Option<Closure<dyn FnMut()>>>>
                            = Rc::new(Cell::new(None));
                let cb      = {
                    let audio_tag   = self.audio_tag.clone();
                    let emitter     = self.emitter.clone();
                    let cb_rc       = cb_rc.clone();
                    Closure::wrap(Box::new(move || {
                        audio_tag.set_onseeked(None);
                        emitter.borrow_mut().emit(&SoundEvent::Rewinded);
                        cb_rc.set(None);
                    }) as Box<dyn FnMut()>)
                };
                self.audio_tag.set_onseeked(Some(cb.as_ref().unchecked_ref()));
                cb_rc.set(Some(cb));
                self.audio_tag.set_current_time(start_time);
                Ok(())
            }
        }
    }

    #[inline(always)]
    pub fn muted(&self) -> bool { self.audio_tag.muted() }

    #[inline(always)]
    pub fn set_muted(&mut self, muted: bool) { self.audio_tag.set_muted(muted); }

    #[inline]
    pub fn pan(&self) -> f32 { self.pan_node.pan().value() }

    #[inline]
    pub fn set_pan(&mut self, pan: f32) { self.pan_node.pan().set_value(pan); }

    #[inline]
    pub fn volume(&self) -> f32 { self.gain_node.gain().value() }

    #[inline]
    pub fn set_volume(&mut self, vol: f32) { self.gain_node.gain().set_value(vol); }
}


impl EventEmitter<SoundEvent, usize> for SoundInstance {
    #[inline]
    fn emit(&mut self, ev: &SoundEvent) {
        self.emitter.borrow_mut().emit(ev);
    }

    #[inline]
    fn subscribe<F: FnMut(&SoundEvent) -> ActionAfterEvent + 'static>(&mut self, ev_type: SoundEvent, listener: F) -> Option<usize> {
        let res = self.emitter.borrow_mut().subscribe(ev_type, listener);
        res
    }

    #[inline]
    fn unsubscribe(&mut self, ev_type: SoundEvent, listener_id: usize) -> bool {
        self.emitter.borrow_mut().unsubscribe(ev_type, listener_id)
    }

    #[inline]
    fn enabled(&self) -> bool { self.emitter.borrow().enabled() }

    #[inline]
    fn set_enabled(&mut self, enabled: bool) {
        self.emitter.borrow_mut().set_enabled(enabled);
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use super::super::*;
    use super::super::context::*;
    use super::super::context::tests::click_promise;
    use wasm_bindgen_test::*;

    static BASE64_MP3_SOUND: &str =
        "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\
        tk+vAwTEM7KqG3GkEBM3QyHRkjaFJPO2TpD9pb0mpZTTmPTBdu8/M/UwLhJJIpyCUUg2cnDTngVKlU//96BC06VGqrai0QgEg4oAEICAPdysBEJvWwCKyI2CQEyN+9y+JYs4hNllYoR64ZyVkUbxif/7\
        MsRTAAdEN3XAJGHw+ArvfphgBoINUJCvJI8taLvFL3vdVqV8u27Fsf5xHpTE225jcO35rb//0xv40xumfbUnHXx/r/6/+6a+vizJaNmFmJ8ug/wWcych1IyEAAAAMxAgLLBQcwAMciGtBwgKrAmRAiPi\
        AAIELFDCScGfoH+kW7zI/klzPKxDMiQNDte3RBkpEIL52KP5DDw+gAD/+zLEWoAN6P1fmJeAASiX7XuMMAH8AD8CFQl4RVQzEptNLBEPIw+CoHHzIsMCscOFng2yaRIyCQhAJ0RB1INFRLaAjYUpuS6Y\
        BXVW464jBURf1gqGvFP+bH+bbSKSZTdH1MTsaD5otwE47OUMyZRYtSRV5xzCAUp6rQYJAba8AteKDVitDnpqfNJk/7coj6ohOhLEqYcVALhVMjMA//swxEGAB9gxf+AkwVD2Cq40BgwyUinOEtaBYSkR\
        CSB8YOlVVVnroHAn1pIQDWK45CoxKFy7DL3ZgdaOJYEtfej3XqHszvLAIHFtWpqADaZzZUJKQpQBk4JdBemOz05U4XlTShdrsMllsoUka/TbKZNiMjNdSyXbX4bfTxVpdIGQikrnfGtSs7gkGr6X8wDS\
        BplINDIAUk5QFucn//syxEaAB9BPYeAkY4EIlOw8Bgx0grkdVhbUoEB2qhUJ2Mu5EtzgqE3OBVEbg2nbllgFIjFQwIUjXCLnfWAixkbgV11qDtbBJohAADDwYMYiJjAkJkiIUrCuSPDhUcSgrAqkIUmu\
        cEq3fujW5mXH1j5fbDXWTXZwyw0RO/jGqLNIiWL1DmklQCaZdwHdUqsImZFg0fKCsMnUzEuxcf/7MsRKAAfQM1XgBYAA75SpMBSMoKxGZYWjILJJCMiwSkWCWFXMMYSNHQqAo5//kr1IPVAUi46+gJAA\
        jNVhqJ1jSFRqSvCx9ksjYTZ6opSehUaQyaghTGzBj44lrJ/qucOly8WNQEj0Gg6klEs9+jkq6gziWNUyNSnrMhQCGDiBS5cpMFhEJAVmssBRKxZZvkQkDoVIM9p4JBQ1IhL/+zLEUIAHkEVJoKTDQOYT\
        pIg0jWgBeokxBJ/EYSVMQU1FMy45OS41VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV\
        VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV//swxFmDxnBI+gGEYkAAADSAAAAEVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV\
        VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV";

    #[wasm_bindgen_test]
    async fn sound_instance_operations() {
        use wasm_bindgen_futures::JsFuture;
        use js_sys::{Function, Promise};
        <JsFuture as From<Promise>>::from(
            click_promise("Click to test sound operations", true)
                .expect("Could not create button for sound instance tests")
                .2
        )
            .await
            .expect("Promise failed");
        let mut ctx     = SoundContext::new_init()
            .await
            .expect("Could not initialize sound context");
        use crate::utils::html::get_document;
        let doc         = get_document()
            .expect("Could not get document element");
        let audio_tag   = doc
            .create_element("audio")
            .and_then(|elem| elem.dyn_into::<HtmlAudioElement>().map_err(|_| JsValue::NULL))
            .expect("Could not create audio element");
        audio_tag.set_id("base64mp3sound");
        let source_tag  = doc
            .create_element("source")
            .and_then(|elem| elem.dyn_into::<HtmlSourceElement>().map_err(|_| JsValue::NULL))
            .expect("Could not create source element");
        source_tag.set_src(&("data:audio/mpeg;codecs=mp3;base64,".to_string() + BASE64_MP3_SOUND));
        assert!(audio_tag.append_with_node_1(&source_tag).is_ok());
        let mut inst    = ctx.create_sound_instance(&audio_tag)
            .expect("Could not create audio instance");
        inst.set_pan(0.5);
        assert_eq!(inst.pan(), 0.5);
        inst.set_looping(true);
        assert!(inst.looping());
        inst.set_volume(0.75);
        assert_eq!(inst.volume(), 0.75);
        inst.set_muted(true);
        assert!(inst.muted());
        inst.set_muted(false);
        assert!(!inst.muted());
        inst.set_looping(false);
        assert!(!inst.looping());
        inst.set_volume(1.);
        assert_eq!(inst.volume(), 1.);
        inst.set_pan(-1.);
        assert_eq!(inst.pan(), -1.);
        let prom_rc     : Rc<Cell<Option<Box<dyn FnMut(Function, Function)>>>>
                        = Rc::new(Cell::new(None));
        let mut prom_cb = {
            let prom_rc     = prom_rc.clone();
            let mut inst    = inst.clone();
            move |resolve: Function, _reject| {
                console_log!("Inside promise constructor");
                let prom_rc     = prom_rc.clone();
                inst.subscribe(SoundEvent::Ended, {
                    move |_| {
                        console_log!("Inside event listener");
                        resolve.call0(&JsValue::NULL).expect("Could not resolve promise");
                        prom_rc.set(None);
                        ActionAfterEvent::RemoveListener
                    }
                }).expect("Unable to subscribe event listener");
            }
        };
        console_log!("If we're here, the event was subscribed");
        let prom        = Promise::new(&mut prom_cb);
        prom_rc.set(Some(Box::new(prom_cb) as Box<dyn FnMut(Function, Function)>));
        assert!(inst.play().is_ok());
        assert!(<JsFuture as From<Promise>>::from(prom).await.is_ok());
    }
}
