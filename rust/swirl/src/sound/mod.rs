//! Module  :   Sound
//! Purpose :   Simple audio services with volume and panning controls.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Wed 11/17/2021 11:37 AM
//! Modified:   Tue 03/22/2022 02:00 PM


pub mod instance;
pub use instance::*;

pub mod context;
pub use context::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAudioElement, HtmlSourceElement};

use crate::utils::{html::get_document, enum_set::make_enum_set};
use crate::events::EventLike;


//  Duplicates an audio element, source sub-elements included.
fn duplicate_audio_element(h_audio: &HtmlAudioElement) -> Result<HtmlAudioElement, JsValue> {
    let doc         = get_document()?;
    let audio_tag   = HtmlAudioElement::new()?;
    audio_tag.set_autoplay(false);
    audio_tag.set_muted(false);
    audio_tag.set_loop(false);
    audio_tag.set_preload("none");
    let children    = h_audio.children();
    for i in 0 .. children.length() {
        let child       = children
            .item(i)
            .ok_or_else(|| JsValue::from_str(&format!(
                "audio::duplicate_audio_element(): cannot access source node #{}", i 
            )))?
            .dyn_into::<HtmlSourceElement>()?;
        let source_tag  = doc
            .create_element("source")?
            .dyn_into::<HtmlSourceElement>()?;
        source_tag.set_src(&child.src());
        source_tag.set_type(&child.type_());
        audio_tag.append_with_node_1(&source_tag)?;
    }
    Ok(audio_tag)
}


//  An audio event.
make_enum_set!{
    SoundEvent ::=
        Playing     <=> "Playing",
        Paused      <=> "Paused",
        Rewinded    <=> "Rewinded",
        Ended       <=> "Ended",
        Seeked      <=> "Seeked",
}

impl EventLike for SoundEvent {
    type EventIdType = Self;
    fn type_(&self) -> Self { *self }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use web_sys::Element;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn audio_tag_and_sources_duplication() {
        let doc         = get_document().expect("Cannot get document element");
        let audio       = HtmlAudioElement::new().expect("Cannot create audio node");
        let src1        = doc
            .create_element("source")
            .and_then(|source| source.dyn_into::<HtmlSourceElement>().map_err(|_| JsValue::NULL))
            .expect("Cannot create source element");
        src1.set_src("whatever.mp3");
        src1.set_type(r#"audio/mpeg; codecs="mp3""#);
        let src2        = doc
            .create_element("source")
            .and_then(|source| source.dyn_into::<HtmlSourceElement>().map_err(|_| JsValue::NULL))
            .expect("Cannot create source element");
        src2.set_src("whatever.ogg");
        src2.set_type(r#"audio/ogg; codecs="vorbis""#);
        audio.append_with_node_1(&src1).expect("could not append source node");
        audio.append_with_node_1(&src2).expect("could not append source node");
        let audio_dup   = duplicate_audio_element(&audio)
            .expect("Duplication failed");
        //console::log_1(&audio_dup);
        assert_eq!(audio_dup.autoplay(), false, "Copy error: autoplay not disabled");
        assert_eq!(audio_dup.muted(), false, "Copy error: element muted");
        assert_eq!(audio_dup.loop_(), false, "Copy error: looping not disabled");
        assert_eq!(audio_dup.preload(), "none", r#"Copy error: preload different from "none""#);
        let mut has_mp3 = false;
        let mut has_ogg = false;
        for i in 0 .. audio_dup.children().length() {
            let child = audio_dup
                .children()
                .item(i)
                .ok_or_else(|| JsValue::from_str(&format!("Couldn't get duplicate's child #{}", i)))
                .and_then(|item: Element| item.dyn_into::<HtmlSourceElement>().map_err(|_| JsValue::NULL))
                .expect("Child cannot be recognized as <source> element");
            if child.src().ends_with("whatever.ogg") && child.type_() == r#"audio/ogg; codecs="vorbis""# {
                has_ogg = true;
            }
            if child.src().ends_with("whatever.mp3") && child.type_() == r#"audio/mpeg; codecs="mp3""# {
                has_mp3 = true;
            }
        }
        assert!(has_mp3 && has_ogg, "Partial duplication");
    }
}
