//! Module  :   Sound Context
//! Purpose :   A "space" for audio playback. It's an
//!             abstraction of the device's sound resources.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Wed 11/24/2021 10:30 AM
//! Modified:   Tue 12/14/2021 03:54 PM


use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use web_sys::{
    //console,
    HtmlAudioElement,
    AudioContext, AudioContextState, AudioDestinationNode,
    GainNode
};

use super::SoundInstance;


//  The inner machinery required for sound-playing.
#[derive(Debug)]
struct SoundContextInner {
    ctx         : AudioContext,
    dest        : Option<AudioDestinationNode>,
    global_gain : Option<GainNode>,
    vol         : f32,
    muted       : bool,
}

/// A context that permits audio playback and other operations.
#[derive(Clone, Debug)]
pub struct SoundContext(Rc<RefCell<SoundContextInner>>);


impl SoundContext {
    const INITIAL_VOL: f32 = 1.;

    /// Creates a sound context. A secondary initialization phase
    /// (asynchronous) should follow creation, as browser contexts
    /// can be activated only as a response to an event.
    pub fn new() -> Result<Self, JsValue> {
        Ok(SoundContext(Rc::new(RefCell::new(SoundContextInner {
            ctx         : AudioContext::new()?,
            dest        : None,
            global_gain : None,
            vol         : Self::INITIAL_VOL,
            muted       : false,
        }))))
    }

    /// Attempts to initialize the context, allowing for playback.
    pub async fn init(&mut self) -> Result<(), JsValue> {
        //console::log_1(&JsValue::from_str("Inside SoundContext::init"));
        let mut this    = self.0.borrow_mut();
        let state       = this.ctx.state();
        if state != AudioContextState::Running {
            //console::log_1(&JsValue::from_str("Context is suspended. Resuming."));
            let prom    = this.ctx.resume()?;
            <JsFuture as From<Promise>>::from(prom).await?;
            let state   = this.ctx.state();
            //console::log_1(&JsValue::from_str("Resuming attempt awaited."));
            if state != AudioContextState::Running {
                return Err(JsValue::from_str("Could not activate audio context."));
            }
            //else {
            //    console::log_1(&JsValue::from_str("Context activated successfully."));
            //}
        }
        //console::log_1(&JsValue::from_str("Either way, context expected to be running at this point."));
        let dest            = this.ctx.destination();
        let global_gain     = this.ctx.create_gain()?;
        global_gain.gain().set_value(Self::INITIAL_VOL);
        global_gain.connect_with_audio_node(&dest)?;
        this.dest           = Some(dest);
        this.global_gain    = Some(global_gain);
        Ok(())
    }

    /// Asynchronous helper to create *and* initialize a context in one move.
    /// Much like `init`, it should be called in response to an event.
    pub async fn new_init() -> Result<Self, JsValue> {
        //console::log_1(&JsValue::from_str("Inside SoundContext::new_init"));
        let mut this    = Self::new()?;
        this.init().await.map(|_| this)
    }

    #[inline]
    pub fn is_initialized(&self) -> bool { self.0.borrow().dest.is_some() }

    #[inline(always)]
    pub fn global_volume(&self) -> f32 { self.0.borrow().vol }

    pub fn set_global_volume(&mut self, vol: f32) -> Result<(), JsValue> {
        let mut this        = self.0.borrow_mut();
        let mut global_gain = this.global_gain.clone();
        match global_gain {
            None                    =>
                Err(JsValue::from_str(
                    "SoundContext::set_global_volume: cannot set volume for unitialized contexts."
                )),
            Some(ref mut gain_node) => {
                this.vol = vol;
                gain_node.gain().set_value(vol);
                Ok(())
            },
        }
    }

    #[inline(always)]
    pub fn globally_muted(&self) -> bool { self.0.borrow().muted }

    pub fn set_globally_muted(&mut self, muted: bool) -> Result<(), JsValue> {
        let mut this        = self.0.borrow_mut();
        let mut global_gain = this.global_gain.clone();
        match global_gain {
            None                    =>
                Err(JsValue::from_str(
                    "SoundContext::set_globally_muted: cannot set muted value for unitialized contexts."
                )),
            Some(ref mut gain_node) => {
                this.muted = muted;
                gain_node.gain().set_value(if muted { 0. } else { this.vol });
                Ok(())
            },
        }
    }

    pub fn create_sound_instance(&mut self, audio_tag: &HtmlAudioElement) -> Result<SoundInstance, JsValue> {
        let this        = self.0.borrow_mut();
        let opt_gain    = this.global_gain.clone();
        if let Some(ref global_gain) = opt_gain {
            let pan_node    = this.ctx.create_stereo_panner()?;
            let gain_node   = this.ctx.create_gain()?;
            let audio_tag   = super::duplicate_audio_element(audio_tag)?;
            crate::utils::html::get_body()
                .expect("Could not access document body")
                .append_with_node_1(&audio_tag)
                .expect("Could not append audio tag");
            let audio_node  = this.ctx.create_media_element_source(&audio_tag)?;
            SoundInstance::new(audio_tag, audio_node, gain_node, pan_node, global_gain)
        }
        else {
            Err(JsValue::from_str("Cannot create sound instances from uninintialized contexts."))
        }
    }
}


#[cfg(test)]
pub mod tests {
    use std::cell::Cell;
    use wasm_bindgen::JsCast;
    use js_sys::Function;
    use web_sys::HtmlButtonElement;
    use super::*;
    use wasm_bindgen_test::*;

    pub fn click_promise(
        btn_label: &str,
        auto_removal: bool
    )
        -> Result<(Option<Box<dyn FnMut()>>, HtmlButtonElement, Promise), JsValue>
    {
        let doc     = crate::utils::html::get_document()?;
        let btn     = doc
            .create_element("button")
            .and_then(|doc| doc
                .dyn_into::<HtmlButtonElement>()
                .map_err(|_| JsValue::from_str("Cannot typecast created button as HtmlButtonElement"))
            )?;
        btn.set_inner_text(btn_label);
        let body = doc.body().ok_or_else(|| JsValue::from_str("Could obtain body element"))?;
        body.append_with_node_1(&btn)?;
        btn.scroll_into_view();
        let prom_rc     : Rc<Cell<Option<Box<dyn FnMut(Function, Function)>>>>
                        = Rc::new(Cell::new(None));
        let onclick_rc  : Rc<Cell<Option<Closure<dyn FnMut()>>>>
                        = Rc::new(Cell::new(None));
        let mut prom_cb = {
            let body        = body.clone();
            let btn         = btn.clone();
            let prom_rc     = prom_rc.clone();
            let onclick_rc  = onclick_rc.clone();
            move |resolve: Function, _reject| {
                let btn         = btn.clone();
                let body        = body.clone();
                let onclick_rc  = onclick_rc.clone();
                let onclick_cb  = {
                    let btn         = btn.clone();
                    let prom_rc     = prom_rc.clone();
                    let onclick_rc  = onclick_rc.clone();
                    let body        = body.clone();
                    Closure::wrap(Box::new(move || {
                        if auto_removal {
                            btn.set_onclick(None);
                            body.remove_child(&btn)
                                .expect("Couldn't remove test button");
                        }
                        resolve.call0(&JsValue::NULL)
                            .expect(
                                r#"swirl::sound::context::tests::button_promise: \
                                call to "resolve" failed"#
                            );
                        if auto_removal {
                            prom_rc.set(None);
                            onclick_rc.set(None);
                        }
                    }) as Box<dyn FnMut()>)
                };
                btn.set_onclick(Some(onclick_cb.as_ref().unchecked_ref()));
                onclick_rc.set(Some(onclick_cb));
            }
        };
        let closer_cb   =
            if auto_removal {
                None
            }
            else {
                let prom_rc     = prom_rc.clone();
                let onclick_rc  = onclick_rc.clone();
                let btn         = btn.clone();
                let body        = body.clone();
                Some(Box::new(move || {
                    btn.set_onclick(None);
                    body.remove_child(&btn)
                        .expect("Couldn't remove test button");
                    prom_rc.set(None);
                    onclick_rc.set(None);
                }) as Box<dyn FnMut()>)
            };
        Ok((closer_cb, btn, Promise::new(&mut prom_cb as &mut dyn FnMut(Function, Function))))
    }

    #[wasm_bindgen_test]
    async fn context_creation_and_later_initialization_and_mute_and_volume() {
        let mut ctx = SoundContext::new().expect("context not created");
        console_log!(
            "Audio context created (uninitialized.) Please ignore \
            warning about context initialization failure above."
        );
        let click = click_promise("Click to initialize\npre-existent sound context", true)
            .unwrap();
        <JsFuture as From<Promise>>::from(click.2).await.unwrap();
        //let init_res = ctx.init().await;
        //assert!(init_res.is_ok(), "Could not initialize SoundContext");
        ctx.init().await.expect("Could not initialize SoundContext");
        assert!(ctx.set_globally_muted(true).is_ok());
        assert!(ctx.globally_muted());
        assert!(ctx.set_globally_muted(false).is_ok());
        assert!(!ctx.globally_muted());
        assert!(ctx.set_global_volume(0.25).is_ok());
        assert_eq!(ctx.global_volume(), 0.25);
    }

    #[wasm_bindgen_test]
    async fn context_creation_and_initialization_one_shot() {
        let click = click_promise("Click to create+initialize\nsound context in one go", true)
            .unwrap();
        <JsFuture as From<Promise>>::from(click.2).await.unwrap();
        let init_res = SoundContext::new_init().await;
        assert!(init_res.is_ok(), "Could not create+initialize active SoundContext");
    }
}
