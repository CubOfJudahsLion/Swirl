//! Module  :   Basic Sound Demo
//! Purpose :   Simple control-based audio demo.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue 12/14/2021 03:03 PM
//! Modified:   Fri 12/17/2021 11:25 AM


use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Index;
use std::rc::Rc;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use js_sys::{Function, Promise};
use web_sys::{
    console,
    HtmlAudioElement,
    HtmlElement,
    HtmlInputElement,
    HtmlOptionElement, HtmlSelectElement,
    HtmlTemplateElement
};
use swirl::*;


/// Main function.
#[wasm_bindgen(start)]
pub async fn run() {
    console::log_1(&JsValue::from_str("SWIRL Sound Demo.\n©2021-22 Multislot Inc."));
    let sounds  = load().await.unwrap();
    let snd_ctx = start_sound().await.unwrap();
    user_player_create(snd_ctx, sounds);
}


//  Resource indexer type
make_enum_set!{
    SoundBites ::=
        AudienceCheer   <=> "audienceCheer",
        BigSplat        <=> "bigSplat",
        CubeOpen        <=> "cubeOpen",
        GlideUp         <=> "glideUp",
        PrizeShows      <=> "prizeShows",
        RevealGlimmer   <=> "revealGlimmer",
        YouHaveWon      <=> "youHaveWon",
        YouWinMusic     <=> "youWinMusic",
}


/// Keeps all sound resources in it.
#[derive(Clone, Debug)]
struct SoundResources {
    pub audience_cheer  : HtmlAudioElement,
    pub big_splat       : HtmlAudioElement,
    pub cube_open       : HtmlAudioElement,
    pub glide_up        : HtmlAudioElement,
    pub prize_shows     : HtmlAudioElement,
    pub reveal_glimmer  : HtmlAudioElement,
    pub you_have_won    : HtmlAudioElement,
    pub you_win_music   : HtmlAudioElement,
}

impl TryFrom<&mut HashMap<SoundBites, Loaded>> for SoundResources {
    type Error = JsValue;

    fn try_from(map: &mut HashMap<SoundBites, Loaded>) -> Result<SoundResources, Self::Error> {
        //
        macro_rules! let_sound {
            ($field:ident = $variant:ident) => {
                let $field = 
                    match map.remove(&SoundBites::$variant) {
                        Some(Loaded::Sound(elem)) =>
                            Ok(elem),
                        _                       =>
                            Err(JsValue::from_str(&format!(
                                "Resource not found: sounds/{}.{{ogg,mp3}}",
                                SoundBites::$variant.to_string()
                            ))),
                    }?;
            };
        }
        //
        let_sound!(audience_cheer = AudienceCheer);
        let_sound!(big_splat      = BigSplat     );
        let_sound!(cube_open      = CubeOpen     );
        let_sound!(glide_up       = GlideUp      );
        let_sound!(prize_shows    = PrizeShows   );
        let_sound!(reveal_glimmer = RevealGlimmer);
        let_sound!(you_have_won   = YouHaveWon   );
        let_sound!(you_win_music  = YouWinMusic  );
        Ok(SoundResources {
            audience_cheer,
            big_splat,
            cube_open,
            glide_up,
            prize_shows,
            reveal_glimmer,
            you_have_won,
            you_win_music,
        })
    }
}

impl Index<SoundBites> for SoundResources {
    type Output = HtmlAudioElement;
    fn index(&self, idx: SoundBites) -> &Self::Output {
        use SoundBites::*;
        match idx {
            AudienceCheer   => &self.audience_cheer,
            BigSplat        => &self.big_splat,
            CubeOpen        => &self.cube_open,
            GlideUp         => &self.glide_up,
            PrizeShows      => &self.prize_shows,
            RevealGlimmer   => &self.reveal_glimmer,
            YouHaveWon      => &self.you_have_won,
            YouWinMusic     => &self.you_win_music,
        }
    }
}


async fn load() -> Result<SoundResources, JsValue> {
    let mut loader  = BasicAssetLoader::<SoundBites>::new();
    for var in SoundBites::ORDERED_VARIANTS {
        loader.add_sound_request(
            *var,
            vec![   "sounds/".to_string() + &var.to_string() + ".mp3"
            ,       "sounds/".to_string() + &var.to_string() + ".ogg"
            ]
        )?;
    }
    let res_rc      : Rc<RefCell<Option<SoundResources>>>
                    = Rc::new(RefCell::new(None));
    let prom_rc     : Rc<Cell<Option<Box<dyn FnMut(Function, Function)>>>>
                    = Rc::new(Cell::new(None));
    let mut prom_cb = {
        let res_rc  = res_rc.clone();
        let prom_rc = prom_rc.clone();
        move |resolve: Function, _reject: Function| {
            let resolve     = Rc::new(resolve);
            let doc         = get_document().unwrap();
            let load_pct    = doc
                .query_selector("label#load-pct")
                .unwrap()
                .expect("Counter label not found.");
            let loader_div  = doc
                .query_selector("div#loader")
                .unwrap()
                .expect("Loader div not found")
                .unchecked_into::<HtmlElement>();
            loader.add_progress_listener(move |percentage|
                load_pct.set_inner_html(&format!("{:.0}", percentage))
            ).unwrap();
            loader.add_loaded_listener({
                let resolve = resolve.clone();
                let res_rc  = res_rc.clone();
                move |map| {
                    res_rc.replace(Some(SoundResources::try_from(map).unwrap()));
                    let timeout_rc  : Rc<Cell<Option<Closure<dyn FnMut()>>>>
                                    = Rc::new(Cell::new(None));
                    let timeout_cb  = {
                        let resolve     = resolve.clone();
                        let timeout_rc  = timeout_rc.clone();
                        let loader_div  = loader_div.clone();
                        Closure::wrap(Box::new(move || {
                            loader_div
                                .style()
                                .set_property("visibility", "hidden")
                                .expect("Could not hide loader pane.");
                            resolve
                                .call0(&JsValue::NULL)
                                .expect("Could not resolve loader promise");
                            timeout_rc.set(None);
                        }) as Box<dyn FnMut()>)
                    };
                    get_window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout_cb.as_ref().unchecked_ref(),
                            500
                        )
                        .expect("Could not set after-load timer");
                    timeout_rc.set(Some(timeout_cb));
                }
            }).unwrap();
            loader.load_all().unwrap();
            prom_rc.set(None);
        }
    };
    let prom        = Promise::new(&mut prom_cb);
    prom_rc.replace(Some(Box::new(prom_cb) as Box<dyn FnMut(Function, Function)>));
    <JsFuture as From<Promise>>::from(prom).await?;
    let res = res_rc
        .borrow()
        .clone();
    res.ok_or_else(|| JsValue::from_str("Resource gathering incomplete."))
}


/// Displays a "start" button and uses the click to return an initialized context.
async fn start_sound() -> Result<SoundContext, JsValue> {
    let doc         = get_document().unwrap();
    let starter_div = doc
        .query_selector("div#starter")?
        .ok_or_else(|| JsValue::from_str("Starter div not found"))?
        .unchecked_into::<HtmlElement>();
    starter_div
        .style()
        .set_property("visibility", "visible")
        .expect("Could not switch to start-button view.");
    let start_btn   = doc
        .query_selector("div#starter button")?
        .ok_or_else(|| JsValue::from_str("Start button not found"))?
        .unchecked_into::<HtmlElement>();
    let prom_rc     : Rc<Cell<Option<Box<dyn FnMut(Function, Function)>>>>
                    = Rc::new(Cell::new(None));
    let mut prom_cb = {
        let prom_rc     = prom_rc.clone();
        let start_btn   = start_btn.clone();
        let starter_div = starter_div.clone();
        move |resolve: Function, _reject: Function| {
            let click_rc    : Rc<Cell<Option<Closure<dyn FnMut()>>>>
                            = Rc::new(Cell::new(None));
            let click_cb    = {
                let start_btn   = start_btn.clone();
                let starter_div = starter_div.clone();
                let click_rc    = click_rc.clone();
                Closure::wrap(Box::new(move || {
                    start_btn.set_onclick(None);
                    starter_div
                        .style()
                        .set_property("visibility", "hidden")
                        .expect("Could not hidden start-button view.");
                    resolve.call0(&JsValue::NULL).expect("Could not resolve click button promise");
                    click_rc.set(None);
                }) as Box<dyn FnMut()>)
            };
            start_btn.set_onclick(Some(click_cb.as_ref().unchecked_ref()));
            click_rc.set(Some(click_cb));
            prom_rc.set(None);
        }
    };
    let prom = Promise::new(&mut prom_cb);
    prom_rc.replace(Some(Box::new(prom_cb)));
    <JsFuture as From<Promise>>::from(prom).await?;
    SoundContext::new_init().await
}


/// Allows the user to create (and destroy) a number of players that
/// can play sounds concurrently.
fn user_player_create(mut ctx: SoundContext, sounds: SoundResources) {
    let doc         = get_document().unwrap();
    let demo_div    = doc
        .query_selector("div#sound-demo")
        .expect("Query for sound playground <div> failed.")
        .expect("Sound playground div now found.")
        .unchecked_into::<HtmlElement>();
    let players_div = doc
        .query_selector("div#sound-demo-players")
        .expect("Query for sound playground <div> failed.")
        .expect("Sound player space div now found.")
        .unchecked_into::<HtmlElement>();
    let template    = doc
        .query_selector("template#player-template")
        .expect("Query for player <template> failed.")
        .expect("Player <template> element not found.")
        .unchecked_into::<HtmlTemplateElement>();
    let selector    = doc
        .query_selector("select#sound-selector")
        .expect("Query for sound selector failed.")
        .expect("Sound <select> element not found.")
        .unchecked_into::<HtmlSelectElement>();
    let create_btn  = doc
        .query_selector("button#create-player")
        .expect("Query for player creator.")
        .expect("Player creation <button> not found.")
        .unchecked_into::<HtmlElement>();
    //  If we've made it this far, we have the html elements we need.
    //  Fill the list of audio bits.
    let sel_opts    = selector.options();
    for &var in SoundBites::ORDERED_VARIANTS {
        let var_repr    = <&'static str as From<SoundBites>>::from(var);
        let option      = HtmlOptionElement::new_with_text_and_value(var_repr, var_repr)
            .expect("Could not create sound bite option.");
        sel_opts
            .add_with_html_option_element(&option)
            .expect("Could not add sound bite to list.");
    }
    //  Now display the players div.
    demo_div
        .style()
        .set_property("visibility", "visible")
        .expect("Could not display sound playground view.");
    //  And add logic the "Add Player" button.
    let create_cb       = {
        let mut player_number = 0u16;
        Closure::wrap(Box::new(move || {
            //  First, obtain the audio sample.
            let sel_str     = selector.value();
            let sel_bite_id = <SoundBites as TryFrom<&str>>::try_from(&sel_str)
                .expect("Could not match selected item to a sound bite.");
            let sel_audio   = &sounds[sel_bite_id];
            let sound_inst  = ctx.create_sound_instance(sel_audio)
                .expect(&format!("Could not create audio sample for \"{}\"", sel_str));
            let sound_inst  = Rc::new(RefCell::new(sound_inst));
            //  Create a template duplicate and add it to the
            //  players' div.
            let first_child = template
                .content()
                .first_element_child()
                .expect("Player template found to be empty.");
            let player      = first_child
                .clone_node()
                .expect("Could not duplicate player template.")
                .dyn_into::<HtmlElement>()
                .expect("Duplicated node is not an HtmlElement.");
            player.set_inner_html(&first_child.inner_html());
            player_number   += 1;
            player.set_id(&format!("player-{}", player_number));
            players_div
                .append_with_node_1(&player)
                .expect("Could not append player to player list.");
            //  Give the player its own logic.
            //  For that, we obtain its controls first.
            let sound_name  = player.query_selector("label#sound-name").unwrap().unwrap().unchecked_into::<HtmlElement>();
            let play_btn    = player.query_selector("button#play").unwrap().unwrap().unchecked_into::<HtmlElement>();
            let loop_chmk   = player.query_selector("input#loop").unwrap().unwrap().unchecked_into::<HtmlInputElement>();
            let stop_btn    = player.query_selector("button#stop").unwrap().unwrap().unchecked_into::<HtmlElement>();
            let pan_slider  = player.query_selector("input#pan").unwrap().unwrap().unchecked_into::<HtmlInputElement>();
            let vol_slider  = player.query_selector("input#vol").unwrap().unwrap().unchecked_into::<HtmlInputElement>();
            let close_btn   = player.query_selector("button#close").unwrap().unwrap().unchecked_into::<HtmlElement>();
            //  Let's label the player.
            sound_name.set_inner_html(&sel_str);
            //  And set callbacks for the controls.
            let callbacks   : Rc<Cell<Option<[Closure<dyn FnMut()>; 6]>>>
                            = Rc::new(Cell::new(None));
            let play_cb     = {
                let sound_inst  = sound_inst.clone();
                Closure::wrap(Box::new(move || {
                    let res = sound_inst.borrow_mut().play();
                    if let Err(e) = res {
                        console::error_1(&e);
                    }
                }) as Box<dyn FnMut()>)
            };
            let loop_cb     = {
                let sound_inst  = sound_inst.clone();
                let loop_chmk       = loop_chmk.clone();
                Closure::wrap(Box::new(move || {
                    sound_inst.borrow_mut().set_looping(loop_chmk.checked());
                }) as Box<dyn FnMut()>)
            };
            let stop_cb     = {
                let sound_inst  = sound_inst.clone();
                Closure::wrap(Box::new(move || {
                    let res = sound_inst.borrow_mut().stop_and_rewind();
                    if let Err(e) = res {
                        console::error_1(&e);
                    }
                }) as Box<dyn FnMut()>)
            };
            let pan_cb      = {
                let sound_inst  = sound_inst.clone();
                let pan_slider  = pan_slider.clone();
                Closure::wrap(Box::new(move || {
                    let slider_val  = pan_slider.value();
                    let pan_res     = <f32 as FromStr>::from_str(&slider_val);
                    match pan_res {
                        Ok(pan) =>
                            sound_inst.borrow_mut().set_pan(pan),
                        _       =>
                            console::error_1(&JsValue::from_str(&format!(
                                "Could not parse pan slider value (\"{}\") into a number.",
                                slider_val
                            ))),
                    }
                }) as Box<dyn FnMut()>)
            };
            let vol_cb      = {
                let sound_inst  = sound_inst.clone();
                let vol_slider  = vol_slider.clone();
                Closure::wrap(Box::new(move || {
                    let slider_val  = vol_slider.value();
                    let vol_res     = <f32 as FromStr>::from_str(&slider_val);
                    match vol_res {
                        Ok(vol) =>
                            sound_inst.borrow_mut().set_volume(vol),
                        _       =>
                            console::error_1(&JsValue::from_str(&format!(
                                "Could not parse volume slider value (\"{}\") into a number.",
                                slider_val
                            ))),
                    }
                }) as Box<dyn FnMut()>)
            };
            let close_cb    = {
                let sound_inst  = sound_inst.clone();
                let callbacks   = callbacks.clone();
                let player      = player.clone();
                let play_btn    = play_btn.clone();
                let loop_chmk   = loop_chmk.clone();
                let stop_btn    = stop_btn.clone();
                let pan_slider  = pan_slider.clone();
                let vol_slider  = vol_slider.clone();
                let close_btn   = close_btn.clone();
                Closure::wrap(Box::new(move || {
                    let _ = sound_inst.borrow_mut().pause();
                    play_btn.set_onclick(None);
                    loop_chmk.set_onclick(None);
                    stop_btn.set_onclick(None);
                    pan_slider.set_onchange(None);
                    vol_slider.set_onchange(None);
                    close_btn.set_onclick(None);
                    player.remove();
                    callbacks.set(None);
                }) as Box<dyn FnMut()>)
            };
            play_btn.set_onclick(Some(play_cb.as_ref().unchecked_ref()));
            loop_chmk.set_onclick(Some(loop_cb.as_ref().unchecked_ref()));
            stop_btn.set_onclick(Some(stop_cb.as_ref().unchecked_ref()));
            pan_slider.set_onchange(Some(pan_cb.as_ref().unchecked_ref()));
            vol_slider.set_onchange(Some(vol_cb.as_ref().unchecked_ref()));
            close_btn.set_onclick(Some(close_cb.as_ref().unchecked_ref()));
            callbacks.set(Some([play_cb, loop_cb, stop_cb, pan_cb, vol_cb, close_cb]));
        }) as Box<dyn FnMut()>)
    };
    create_btn.set_onclick(Some(create_cb.as_ref().unchecked_ref()));
    create_cb.forget();
}
