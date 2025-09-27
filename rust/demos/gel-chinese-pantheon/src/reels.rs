//! # Module        :   _Slot Reel_
//! - **Purpose**   :   A single rotating reel with N symbols on it.
//! - **Version**   :   0.2.1
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Tue Mar  9 08:57:50 AM CST 2021
//! - **Modified**  :   Wed 18 May 2022 08:41:39 AM CST


use std::cell::RefCell;
use std::fmt::{self, Display, Debug, Formatter};
use std::ops::Index;
use std::rc::Rc;

use wasm_bindgen::{JsValue, JsCast, closure::Closure, throw_val, UnwrapThrowExt};
use js_sys::{Function, Promise};
#[cfg(debug_assertions)]
use web_sys::console;

use swirl::*;

use super::assets::*;
use super::game_builder::GameVisuals;


#[derive(Clone, Debug)]
pub enum ReelError {
    SymbolCountTooSmall,
    SymbolCountTooBig,
    StripTooTall,
    InitialSymNotFound(String),
    UnknownSymbol(char),
    IconsTooTall,
    IconSizeZero,
    PositionSettingFailed,
    //OutOfBounds,
}

impl Display for ReelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        use ReelError::*;
        match self {
            &SymbolCountTooBig          => f.write_str("Symbol count too big"),
            &SymbolCountTooSmall        => f.write_str("Symbol count too small"),
            &StripTooTall               => f.write_str("Proposed symbol strip would be too tall"),
            &InitialSymNotFound(ref s)  => f.write_fmt(format_args!("Initial symbol sequence \"{}\" not found in reel", &s)),
            &UnknownSymbol(ch)          => f.write_fmt(format_args!("Character '{}' is not a valid symbol", ch)),
            &IconsTooTall               => f.write_str("Icon height is too massive"),
            &IconSizeZero               => f.write_str("Icon height zero is not allowed"),
            &PositionSettingFailed      => f.write_str("Failed setting symbol position"),
            //ReelError::OutOfBounds          => f.write_str("Out of bounds"),
        }
    }
}

impl From<ReelError> for JsValue {
    fn from(reel_err: ReelError) -> Self {
        JsValue::from_str(&format!("{}", reel_err))
    }
}


//  Data for the reel to render a symbol sequence out of place.
#[derive(Clone, Copy, Debug)]
struct SymbolOverride {
    sym_pos : i16,  //  Symbol position to be duplicated
    replace : i16,  //  Overriden position (i.e., where the duplicates will appear)
}


//  Data required for a spinning reel
struct ReelMotion {
    interval_handle : i32,
    frames_left     : Option<i16>,
    #[allow(dead_code)]
    anim_closure    : Closure<dyn FnMut()>,
    sym_over        : Option<SymbolOverride>,
}

impl ReelMotion {
    const PIXELS_PER_FRAME: i16 =  60;  //  This may vary in the future,
                                        //  or disappear (might not be constant)
}


//  Real structure of the reel. The actual `Reel` type is
//  a share wrapper around this.
struct ReelInner {
    icon_height         : i16,
    icon_count          : i16,
    all_icons           : Box<[SceneGraphNode<f32>]>,
    all_icons_str       : String,
    visible_icon_count  : i16,
    strip               : SceneGraphFrame<f32>,
    pos                 : i16,
    motion              : Option<ReelMotion>,
}

/// Represents a single spinning reel (column.) Handles animation
/// by itself, but not clipping (it is more efficient to implement
/// clipping for a *set* of reels, e.g., with an eraser drawable.
/// For now, it is a single-speed reel without startup or stop/
/// effects. Might be added in the future.
#[derive(Clone)]
pub struct Reel(Shared<ReelInner>);


impl Index<i16> for Reel {
    type Output = SceneGraphNode<f32>;
    fn index(&self, idx: i16) -> &Self::Output {
        let idx = if let Some(over) = self.0.motion.as_ref().and_then(|motion| motion.sym_over) {
            if over.replace <= idx && idx < over.replace + self.0.visible_icon_count {
                (idx + over.sym_pos - over.replace + self.0.icon_count) % self.0.icon_count
            }
            else {
                idx
            }
        }
        else {
            idx
        };
        &self.0.all_icons[idx as usize]
    }
}


impl Reel {
    #[inline(always)]
    pub fn len(&self) -> i16 {
        self.0.icon_count
    }

    fn search_symbol_sequence(to_search: &str, search_in: &str) -> Option<usize> {
        let to_search_chars = to_search.chars().collect::<Vec<char>>();
        let search_in_chars = search_in.chars().collect::<Vec<char>>();
        let to_search_len   = to_search_chars.len();
        let search_in_len   = search_in_chars.len();
        if to_search_len == 0 || to_search_len > search_in_len {
            None
        }
        else {
            for i in 0 .. search_in_len {
                if to_search_chars
                    .iter()
                    .enumerate()
                    .all(|(j, ch)| *ch == search_in_chars[(i + j) % search_in_len])
                {
                    return Some(i);
                }
            }
            None
        }
    }

    pub fn new(
        assets          : &AssetsStruct,
        icon_height     : u16,
        icons           : &str,
        initial_icons   : &str
    )
        -> Result<Reel, ReelError>
    {
        let letters             = icons.chars().collect::<Vec<char>>();
        let icon_count  : i16   = letters.len().try_into().map_err(|_| ReelError::SymbolCountTooBig)?;
        let icon_height : i16   = icon_height.try_into().map_err(|_| ReelError::IconsTooTall)?;
        let visible_icon_count  = <i16 as TryFrom<usize>>::try_from(initial_icons.chars().count())
            .map_err(|_| ReelError::SymbolCountTooBig)?;
        if icon_height == 0 {
            Err(ReelError::IconSizeZero)
        }
        else if visible_icon_count == 0 {
            Err(ReelError::SymbolCountTooSmall)
        }
        else if icon_count < visible_icon_count + 1 {
            Err(ReelError::SymbolCountTooSmall)
        }
        else if (icon_count as i32) * (icon_height as i32) > i16::MAX as i32 {
            Err(ReelError::StripTooTall)
        }
        else {
            let pos             = Self::search_symbol_sequence(initial_icons, icons)
                .ok_or_else(|| ReelError::InitialSymNotFound(initial_icons.to_string()))
                .and_then(|pos|
                    <i16 as TryFrom<usize>>::try_from(pos * icon_height as usize)
                        .map_err(|_| ReelError::SymbolCountTooBig)
                )?;
            let mut all_icons   = Vec::<SceneGraphNode<f32>>::with_capacity(letters.len());
            for letter in letters {
                all_icons.push(
                    SceneGraphLeaf::<f32>::new_node(
                        &Sprite::new(
                            assets[&letter
                                .try_into()
                                .map_err(|_| ReelError::UnknownSymbol(letter))?
                            ]
                            .clone()
                        )
                    )
                );
            }
            let strip           = SceneGraphFrame::<f32>::new();
            let mut reel        = Reel(Shared::new(ReelInner {
                icon_height,
                icon_count,
                all_icons       : all_icons.into_boxed_slice(),
                all_icons_str   : icons.to_string(),
                visible_icon_count,
                strip,
                pos,
                motion  : None,
            }));
            reel.set_position(pos).map_err(|_| ReelError::PositionSettingFailed)?;
            Ok(reel)
        }
    }

    pub fn scene_frame(&self) -> SceneGraphFrame<f32> {
        self.0.strip.clone()
    }

    #[inline(always)]
    pub fn generate_scene_node(&self) -> SceneGraphNode<f32> {
        SceneGraphNode::Frame(self.scene_frame())
    }

    /// Set the viewing position relative to the reel. It's not
    /// like setting the y coordinate of a hypothetical scene node,
    /// but rather like setting the position of a vertical scroll
    /// bar in an image viewer, except that this scroll bar
    /// allows for negative coordinates.
    #[inline(always)]
    pub fn position(&self) -> i16 { self.0.pos }

    pub fn set_position(&mut self, pos: i16) -> Result<(), JsValue> {
        let strip_height    =   self.len() * self.0.icon_height;
        self.0.pos          =   pos % strip_height;
        if self.0.pos < 0 {
            self.0.pos += strip_height;
        }
        self.0.strip.remove_all_children();
        let start_symbol    =   pos / self.0.icon_height;
        let sym_offset      =   pos % self.0.icon_height;
        let sym_range       =   start_symbol
                            ..  (   start_symbol
                                +   self.0.visible_icon_count
                                +   (if sym_offset == 0 { 0 } else { 1 })
                                );
        for (vis_idx, sym_idx) in sym_range.enumerate() {
            //let mut symbol = self.0.all_icons[(sym_idx % self.0.icon_count) as usize].clone();
            let mut symbol = self[sym_idx % self.0.icon_count].clone();
            symbol.set_y(vis_idx as i16 * self.0.icon_height - sym_offset);
            self.0.strip
                .append_child(symbol)
                .map_err(|sg_err| JsValue::from_str(sg_err.into()))?;
                //.map_err(|_| JsValue::from_str(
                //    "swirl::gel::reels::Reel::set_position(): \
                //     could not append symbol to reel."
                //))?;
        }
        Ok(())
    }

    pub fn start_spin(&mut self) {
        match self.remove_spin_listeners()
            .and_then(|_| get_window())
            .and_then(|wnd| {
                let anim_closure = {
                    let mut this        = self.clone();
                    use HermiteTangentMode::*;
                    let startup_time    = 250.0f32;
                    let startup_height  =  20.0f32;
                    let mut anim_curve  = HermitePiecewiseInterpolator::<f32, f32>::new(
                        FiniteDifference,
                        &[
                            ( -GameVisuals::MSPF as f32,                0.0,                                  None),
                            (  0.0,                                     0.0,                                  None),
                            (  startup_time / 2.,                       startup_height,                       None),
                            (  startup_time,                            0.0,                                  None),
                            (  startup_time + GameVisuals::MSPF as f32, -ReelMotion::PIXELS_PER_FRAME as f32, None),
                        ]
                    ).unwrap();
                    let mut curr_time   = 0.0f32;
                    let original_pos    = this.position();
                    Closure::wrap(Box::new(move || {
                        this.set_position(
                            if curr_time <= startup_time {
                                let interpolation = anim_curve.interpolate(curr_time);
                                curr_time += GameVisuals::MSPF as f32;
                                original_pos + interpolation as i16
                            }
                            else {
                                this.position() - ReelMotion::PIXELS_PER_FRAME
                            }
                        )
                        .expect_throw("Reel positioning failed.");
                    }) as Box<dyn FnMut()>)
                };
                wnd.set_interval_with_callback_and_timeout_and_arguments_0(
                    anim_closure.as_ref().unchecked_ref(),
                    GameVisuals::MSPF
                )
                .map(move |interval_handle| (interval_handle, anim_closure))
            })
        {
            Ok((interval_handle, anim_closure)) => {
                self.0.borrow_mut().motion = Some(ReelMotion {
                    interval_handle,
                    frames_left : None,
                    anim_closure,
                    sym_over    : None,
                });
            },
            Err(js_value)   => throw_val(js_value),
        }
    }

    fn remove_spin_listeners(&mut self) -> Result<(), JsValue> {
        let is_some = if let &Some(ReelMotion {interval_handle, ..}) = &self.0.motion {
            get_window()
                .map(|wnd| wnd.clear_interval_with_handle(interval_handle))?;
            true
        }
        else {
           false
        };
        if is_some {
            self.0.motion = None;
        }
        Ok(())
    }

    pub fn stop_spin(&mut self, stop_symbols: String, instant: bool, _flags: Option<Box<[String]>>) -> Promise {
        //  Find the symbol or prepare a proper error message.
        match Reel::search_symbol_sequence(&stop_symbols, &self.0.all_icons_str)
            .ok_or_else(|| JsValue::from_str(&format!(
                r#"Could not find sequence "{}" in reel "{}""#,
                &stop_symbols,
                &self.0.all_icons_str
            )))
            .and_then(|sym_idx| self
                .remove_spin_listeners()
                .map(|_| sym_idx)
                .map_err(|_| JsValue::from_str("Removing previous reel spinner failed."))
            )
        {
            //  The symbol exists:
            Ok(sym_idx) => {
                //  We use i16 'round these parts, stranger.
                let target_sym = sym_idx as i16;
                //  Instant stop? Then just do it.
                if instant {
                    match self.set_position(target_sym * self.0.icon_height) {
                        Ok(_)   => Promise::resolve(&JsValue::UNDEFINED),
                        _       => Promise::reject(&JsValue::from_str("Could not set final reel position")),
                    }
                }
                else {
                    //  Otherwise, we need to time things carefully.
                    let (injected_idx, frames_left): (Option<i16>, i16) = {
                        let strip_height    =   self.0.icon_count * self.0.icon_height;
                        let curr_pos        =   self.position();
                        //  Minimum possible distance we can go to place the injection
                        //  point without overlapping with on-screen symbols.
                        let min_distance_to_inj     =   self.0.icon_height * self.0.visible_icon_count
                                                    +   curr_pos % self.0.icon_height;
                        //  Target symbol index from current, could be negative.
                        let injected_sym_rel        =   (curr_pos - min_distance_to_inj) / self.0.icon_height;
                        //  Injected index, corrected for overlap
                        //  (not always needed, positive when it is.)
                        let mut injected_sym_idx    =   {
                            //  The first task is to prevent overlap between the real and injected
                            //  position, which results in trying to insert a symbol twice. These
                            //  differences will tell us if there's overlap either way.
                            let upward_diff         =   injected_sym_rel - target_sym
                                                    +   if target_sym > injected_sym_rel { self.0.icon_count } else { 0 };
                            let downward_diff       =   target_sym - injected_sym_rel
                                                    +   if target_sym < injected_sym_rel { self.0.icon_count } else { 0 };
                            //  If the actual position is the same as the injected one,
                            //  or is a very short distance from it going up, we use it
                            //  and forgo injecting.
                            if upward_diff <= self.0.visible_icon_count {
                                None
                            }
                            //  Now if the injected position is slightly above the target
                            //  one, we go up a few frames to prevent that overlap.
                            else if downward_diff <= self.0.visible_icon_count {
                                Some(
                                        (   injected_sym_rel
                                        -   self.0.visible_icon_count
                                        -   1
                                        +   downward_diff
                                        +   self.0.icon_count
                                        )
                                    %   self.0.icon_count
                                )
                            }
                            //  Otherwise, there's no overlap, and we keep the injected
                            //  position as is.
                            else {
                                Some((injected_sym_rel + self.0.icon_count) % self.0.icon_count)
                            }
                        };
                        //  At this point we know a calculated, positive target position,
                        //  whether injected or not.
                        let mut target_idx          =   injected_sym_idx.unwrap_or(target_sym);
                        //  We calculate the target pixel position going up relative to current.
                        let mut target_pos          =   target_idx * self.0.icon_height;
                        if target_pos > curr_pos { target_pos -= strip_height; }
                        //  One more possible correction. We don't want duplicated symbols, 
                        //  so we go a few squares up if we happen to collide.
                        let icon_chars              =   self.0.all_icons_str.chars();
                        while   icon_chars.clone().nth(((target_idx + 3) % self.0.icon_count) as usize)
                            ==  icon_chars.clone().nth(( sym_idx    + 2) % self.0.icon_count  as usize)
                        {
                            target_idx           =
                                if target_idx < self.0.visible_icon_count {
                                    self.0.icon_count + target_idx    - self.0.visible_icon_count
                                }
                                else {
                                    target_idx                        - self.0.visible_icon_count
                                };
                            target_pos          -= self.0.icon_height * self.0.visible_icon_count;
                            injected_sym_idx     = Some(target_idx);
                        }
                        let frames_left             =   (   curr_pos
                                                        -   target_pos
                                                        +   ReelMotion::PIXELS_PER_FRAME
                                                        -   1
                                                        )
                                                    /   ReelMotion::PIXELS_PER_FRAME;
                        (injected_sym_idx, frames_left)
                    };
                    #[cfg(debug_assertions)]
                    console::log_1(&JsValue::from_str(&format!(
                        "Real: {}, injected: {}",
                        target_sym,
                        injected_idx
                            .map(|idx| format!("Some({})", idx))
                            .unwrap_or_else(|| "None".to_string())
                    )));
                    //  With this information, we can create the index override and
                    //  interval callbacks.
                    let prom_mem        : Rc<RefCell<Option<Box<dyn FnMut(Function, Function)>>>>
                                        = Rc::new(RefCell::new(None));
                    let mut prom_fn     = {
                        let this        = self.clone();
                        let prom_mem    = prom_mem.clone();
                        move |resolve: Function, reject: Function| {
                            let anim_closure    = {
                                let resolve         = resolve.clone();
                                let reject          = reject.clone();
                                let prom_mem        = prom_mem.clone();
                                let mut this        = this.clone();
                                let bounce_time     = 500.0f32;
                                let bounce_height   =  20.0f32;
                                let mut intp_time   =   0.0f32;
                                let mut anim_curve  = HermitePiecewiseInterpolator::<f32, f32>::new(
                                    HermiteTangentMode::<f32>::Cardinal(0.5),
                                    &[
                                        (-GameVisuals::MSPF as f32,                      -ReelMotion::PIXELS_PER_FRAME as f32,      None),
                                        ( 0.0,                                            0.0,                                      None),
                                        ( bounce_time / 2.,                               bounce_height,                            None),
                                        ( bounce_time,                                    0.0,                                      Some(HermiteTangentMode::<f32>::KonachekBartels {
                                                                                                                                        tension     :  1.,
                                                                                                                                        continuity  : -1.,
                                                                                                                                        bias        :  0.,
                                                                                                                                    })),
                                        ( bounce_time + GameVisuals::MSPF as f32,        -ReelMotion::PIXELS_PER_FRAME as f32,      None),
                                        ( bounce_time + (GameVisuals::MSPF * 2) as f32, (-ReelMotion::PIXELS_PER_FRAME * 2) as f32, None),
                                    ],
                                ).unwrap();
                                let mut sym_reached = false;
                                Closure::wrap(Box::new(move || {
                                    let finished = {
                                        if !sym_reached {
                                            sym_reached = match this.0.motion
                                                .as_mut()
                                                .ok_or_else(|| JsValue::from_str("Could not access motion data"))
                                                .and_then(|motion| motion.frames_left.as_mut()
                                                    .ok_or_else(|| JsValue::from_str(
                                                        "Could not access remaining frame count for stopper"
                                                    ))
                                                )
                                            {
                                                Ok(frames_left)  => {
                                                    *frames_left -= 1;
                                                    if *frames_left == 0 {
                                                        true
                                                    }
                                                    else {
                                                        this.set_position(this.position() - ReelMotion::PIXELS_PER_FRAME)
                                                            .unwrap_throw();
                                                        false
                                                    }
                                                },
                                                Err(js_err) => {
                                                    reject.call1(&JsValue::NULL, &js_err)
                                                        .map_err(|_| JsValue::from_str(
                                                            "Could not reject stopping animation promise"
                                                        ))
                                                        .unwrap_throw();
                                                    true
                                                }
                                            };
                                            false
                                        }
                                        else {
                                            intp_time += GameVisuals::MSPF as f32;
                                            if intp_time >= bounce_time {
                                                true
                                            }
                                            else {
                                                let interp   = anim_curve.interpolate(intp_time);
                                                //console::log_1(&JsValue::from_str(&format!(
                                                //    "HermIntp.time:{}.val:{}",
                                                //    intp_time,
                                                //    interp
                                                //)));
                                                this.set_position(target_sym * this.0.icon_height + interp as i16)
                                                    .unwrap_throw();
                                                false
                                            }
                                        }
                                    };
                                    let pos_set_outcome =
                                        if finished {
                                            get_window()
                                                .and_then(|wnd| this.0.motion
                                                    .as_ref()
                                                    .map(|motion| motion.interval_handle)
                                                    .ok_or_else(|| JsValue::from_str("Could not access motion data"))
                                                    .map(move |interval_handle| (wnd, interval_handle))
                                                )
                                                .and_then(|(wnd, interval_handle)| {
                                                    wnd.clear_interval_with_handle(interval_handle);
                                                    this.set_position(target_sym * this.0.icon_height)
                                                })
                                                .and_then(|_| resolve.call0(&JsValue::UNDEFINED)
                                                    .map(|_| Some(&mut this.0.motion))
                                                    .map_err(|_| JsValue::from_str(
                                                        "Could not resolve reel-stopper promise"
                                                    ))
                                                )
                                        }
                                        else {
                                            Ok(None)
                                        };
                                    match pos_set_outcome {
                                        Err(js_err)         => {
                                            reject.call1(&JsValue::NULL, &js_err)
                                                .map_err(|_| JsValue::from_str(
                                                    "Failed to reject stop animation promise"
                                                ))
                                                .unwrap_throw();
                                            prom_mem.replace(None);
                                        },
                                        Ok(None)            => {
                                            ()
                                        },
                                        Ok(Some(motion))    => {
                                            prom_mem.replace(None);
                                            *motion = None;
                                        },
                                    }
                                }) as Box<dyn FnMut()>)
                            };
                            match get_window()
                                .and_then(|wnd|
                                    wnd.set_interval_with_callback_and_timeout_and_arguments_0(
                                        anim_closure.as_ref().unchecked_ref(),
                                        GameVisuals::MSPF
                                    )
                                )
                            {
                                Ok(interval_handle) => {
                                    this.0.borrow_mut().motion = Some(ReelMotion {
                                        interval_handle,
                                        frames_left :   Some(frames_left),
                                        anim_closure,
                                        sym_over    :   injected_idx.map(|inj_idx|
                                                            SymbolOverride {
                                                                sym_pos: target_sym,
                                                                replace: inj_idx,
                                                            }
                                                        ),
                                    });
                                },
                                Err(js_value)       => {
                                    throw_val(js_value);
                                },
                            }
                        }
                    };
                    let prom = Promise::new(&mut prom_fn);
                    prom_mem.replace(Some(Box::new(prom_fn)));
                    prom
                }
            },
            Err(js_err) => {
                Promise::reject(&js_err)
            }
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_spinning(&self) -> bool {
        self.0.motion.is_some()
    }
}
