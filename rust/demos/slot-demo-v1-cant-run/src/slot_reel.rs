//! Module  :   Slot Reel
//! Purpose :   A single rotating reel with N symbols on it.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Mar  9 08:57:50 AM CST 2021
//! Modified:   Tue Jul  6 02:44:19 PM CST 2021


use std::fmt::{self, Display, Debug, Formatter};

use wasm_bindgen::{JsValue, JsCast, closure::Closure}; //, UnwrapThrowExt};
use web_sys::{window /*, console*/};

use swirl::*;
use swirl::utils::*;
use swirl::math::Point;
//use swirl::math::interpolation::*;

use super::res;


#[derive(Debug)]
pub enum ReelError {
    SymbolCountTooSmall,
    SymbolCountTooBig,
    //OutOfBounds,
}

impl Display for ReelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            ReelError::SymbolCountTooBig    => f.write_str("Symbol count too big"),
            ReelError::SymbolCountTooSmall  => f.write_str("Symbol count too small"),
            //ReelError::OutOfBounds          => f.write_str("Out of bounds"),
        }
    }
}


#[allow(dead_code)]
struct ReelMotion {
    interval_handle : i32,
    anim_closure    : Closure<dyn FnMut()>,
}


//  A symbol representation that allows for multi-row/column symbols.
#[derive(Clone)]
pub enum SymbolSpace {
    Single(res::img::Icon, SceneGraphNode, Point<i32>),             // Unique row, with offset for symbol excesses.
    Multiple(res::img::Icon, SceneGraphNode, u8, u8, Point<i32>),   // Icon, spanned rows up and below and offset.
    Blank,                                                          // Uninhabited blocks. This allows horizontal
                                                                    // spanning and many other scenarios.
}

impl Debug for SymbolSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use SymbolSpace::*;
        match self {
            &Single(icon, _, ref ofs)                   =>
                f.write_str(&format!("Single({:?}(@{:?}x{:?}))", icon, ofs.x, ofs.y)),
            &Multiple(icon, _, above, below, ref ofs)   =>
                f.write_str(&format!("Multiple({:?}(^{:?}v{:?})(@{:?}x{:?}))", icon, above, below, ofs.x, ofs.y)),
            &Blank                                      =>
                f.write_str("Blank"),
        }
    }
}


pub struct ReelS {
    strip_icon_count    : i32,
    icon_height         : i32,
    icon_sprites        : Box<[SymbolSpace]>,
    //visible_icon_count  : i32,
    strip               : SceneGraphFrame,
    pos                 : i32,
    motion              : Option<ReelMotion>,
}

pub struct Reel(Shared<ReelS>);

impl Clone for Reel {
    #[inline]
    fn clone(&self) -> Self { Reel(self.0.clone()) }
}


impl Reel {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.0.icon_sprites.len()
    }

    pub fn new(x: i32, y: i32, icon_height: u16, icon_width: u16, visible_icon_count: u8, icons: Vec<SymbolSpace>) -> Result<Reel, ReelError> {
        let strip_icon_count    = icons.len() as i32;
        if strip_icon_count as usize != icons.len() {
            Err(ReelError::SymbolCountTooBig)?;
        }
        let icon_height         = icon_height as i32;
        let icon_width          = icon_width as i32;
        let visible_icon_count  = visible_icon_count as i32;
        if strip_icon_count < visible_icon_count + 2 {
            Err(ReelError::SymbolCountTooSmall)
        }
        else {
            let strip               = SceneGraphFrame::new(
                Some(Point::new(x, y)),
                Some(Rect::new(0, 0 - 5 * icon_width, icon_height * visible_icon_count, icon_width + 5 * icon_width)),
                None
            );
            //let mut icon_sprites    = Vec::<SymbolSpace>::with_capacity(strip_icon_count as usize);
            //let mut sym_num         = -2;
            //while sym_num < strip_icon_count - 2 {
            //    let mut num     : usize;
            //    let mut img_res : res::img::Icon;
            //    let mut height  : i32;
            //    let pos         : Point<i32>;
            //    loop {
            //        num     = (random() * res::img::Icon::VARIANT_COUNT as f64).floor() as usize;
            //        img_res = res::img::Icon::ORDERED_VARIANTS[num];
            //        height  = match img_res {
            //            res::img::Icon::A   |
            //            res::img::Icon::B   |
            //            res::img::Icon::C   |
            //            res::img::Icon::D   |
            //            res::img::Icon::X   |
            //            res::img::Icon::W   => 3,
            //            res::img::Icon::E   |
            //            res::img::Icon::F   |
            //            res::img::Icon::G   |
            //            res::img::Icon::H   => 1,
            //        };
            //        if height + sym_num - 1 >= strip_icon_count as i32 - 2 {
            //            continue;
            //        }
            //        pos     = match img_res {
            //            res::img::Icon::E   => Point::<i32>::new(-18, -39),
            //            res::img::Icon::F   => Point::<i32>::new(-19, -20),
            //            res::img::Icon::G   => Point::<i32>::new(-19, -21),
            //            res::img::Icon::H   => Point::<i32>::new(-24, -36),
            //            res::img::Icon::A   |
            //            res::img::Icon::B   |
            //            res::img::Icon::C   |
            //            res::img::Icon::D   |
            //            res::img::Icon::X   |
            //            res::img::Icon::W   => Point::<i32>::new(  0,   0),
            //        };
            //        break;
            //    }
            //    let img     = images[img_res].clone();
            //    let graph   = SceneGraphFrame::new_node(
            //        Some(Point::<i32>::new(0, sym_num * icon_height)),
            //        None,
            //        Some(&[SceneGraphLeaf::new_node(Sprite::new(img), Some(pos))])
            //    );
            //    if height == 1 {
            //        icon_sprites.push(SymbolSpace::Single(graph));
            //    }
            //    else {
            //        icon_sprites.extend_from_slice(
            //            &[  SymbolSpace::Multiple(graph.clone(), 0, 2)
            //            ,   SymbolSpace::Multiple(graph.clone(), 1, 1)
            //            ,   SymbolSpace::Multiple(graph        , 2, 0)
            //            ]
            //        );
            //    }
            //    sym_num += height;
            //    //if i <= Self::VISIBLE_REELS as i32 {
            //    //    let _ = strip.append(graph.clone());
            //    //}
            //}
            let mut reel = Reel(Shared::new(ReelS {
                strip_icon_count,
                icon_height,
                icon_sprites: icons.into_boxed_slice(),
                strip,
                //visible_icon_count,
                pos     : 0,
                motion  : None,
            }));
            reel.set_position(0);
            Ok(reel)
        } 
    }

    pub fn scene_frame(&self) -> SceneGraphFrame {
        self.0.strip.clone()
    }

    #[inline(always)]
    pub fn scene_node(&self) -> SceneGraphNode {
        SceneGraphNode::Frame(self.scene_frame())
    }

    //pub fn position(&self) -> i32 { self.0.pos }

    pub fn set_position(&mut self, pos: i32) {
        let icon_height         = self.0.icon_height;
        let icon_count          = self.0.strip_icon_count as i32;
        let strip_pixel_size    = icon_count * icon_height;
        self.0.pos              = pos % strip_pixel_size;
        if self.0.pos < 0 {
            self.0.pos += strip_pixel_size;
        }
        let upper_pos           = (strip_pixel_size - self.0.pos) % strip_pixel_size;
        let top_icon            = upper_pos / icon_height;
        let icon_hidden         = upper_pos % icon_height;
        self.0.strip.children_mut().clear();
        for i in -1_i32 .. (if icon_hidden == 0 { 4 } else { 5 }) {
            let idx         = ((i + top_icon + icon_count) % icon_count as i32) as usize;
            let mut row_set = self.0.icon_sprites[idx].clone();
            match row_set {
                SymbolSpace::Single(_, ref mut sprite, ref pivot)               => { 
                    sprite.set_x_and_y(pivot.x, i * icon_height - icon_hidden + pivot.y);
                   //sprite.set_y(i * icon_height - icon_hidden);
                    self.0.strip.children_mut().push(sprite.clone());
                },
                SymbolSpace::Multiple(_, ref mut sprite, above, _, ref pivot)   => {
                    if i == -1 || above == 0 {
                        sprite.set_x_and_y(pivot.x, (i - above as i32) * icon_height - icon_hidden + pivot.y);
                        //sprite.set_y((i - above as i32) * icon_height - icon_hidden);
                        self.0.strip.children_mut().push(sprite.clone());
                    }
                },
                SymbolSpace::Blank                                              =>
                    (),
            }
        }
    }

    /// Makes the reel spin for a fixed amount of time.
    pub fn start_spin<F>(&mut self, every_ms: i32, mut callback: F) -> Result<(), JsValue>
        where   F:  FnMut() + 'static
    {
        self.stop_spin()?;
        let wnd             = window()
            .ok_or_else(|| JsValue::from_str("Cannot find main window."))?;
        let mspf            = every_ms as f32;
        let mut self_copy   = self.clone();
        let bottom          = 60.0 * self.0.icon_height as f32;
        let initial_pos     = self.0.pos;
        use HermiteTangentMode::*;
        let mut anim_curve  = HermitePiecewiseInterpolator::<f32, f32>::new(
            FiniteDifference,
            &[
                (        -mspf,            0.0, None),
                (          0.0,            0.0, None),
                (  mspf *  5.0,           75.0, Some(KonachekBartels {continuity: 0.0, tension: 0.5, bias: 0.0})),
                (       3425.0,  bottom + 40.0, Some(Linear)),
                (       3525.0,  bottom + 10.0, None),
                (       3625.0,         bottom, None),
                (       3700.0,         bottom, None),
            ]
            //HermiteTangentMode::FiniteDifference,
            //&[
            //    (-mspf,    0.0,         None),
            //    (   0.0,   0.0,         None),
            //    (3525.0, bottom + 20.0, None),
            //    (3625.0, bottom,        None),
            //    (3700.0, bottom,        None),
            //]
        ).map_err(|_| JsValue::from_str("Couldn't create animation curve."))?;
        let mut time        = 0.0_f32;
        let anim_closure    = Closure::wrap(Box::new(move || {
            let new_pos = anim_curve.interpolate(time);
            let int_pos = initial_pos + new_pos.round() as i32;
            //console::log_1(&JsValue::from_str(&format!("interpolated: {}, integer: {}", new_pos, int_pos)));
            self_copy.set_position(int_pos);
            //self_copy.set_position(self_copy.position() + self_copy.0.icon_height / 2);
            time    += mspf;
            if time > 3625.0 {
                //  anim_curve is an approximating sampler. Ensure we actually stop at the very end.
                self_copy.set_position(initial_pos + bottom as i32);
                let _ = self_copy.stop_spin();
                callback();
            }
        }) as Box<dyn FnMut()>);
        let interval_handle = wnd.set_interval_with_callback_and_timeout_and_arguments_0(
            anim_closure.as_ref().unchecked_ref(), every_ms
        )?;
        self.0.motion = Some(ReelMotion {interval_handle, anim_closure});
        Ok(())
    }

    //  Stops the reel.
    fn stop_spin(&mut self) -> Result<(), JsValue> {
        match self.0.motion {
            Some(ReelMotion {interval_handle, anim_closure: _}) => {
                window()
                    .ok_or_else(|| JsValue::from_str("Cannot find main window."))?
                    .clear_interval_with_handle(interval_handle);
                self.0.motion = None;
                Ok(())
            },
            _                                                   =>
                Ok(()),
        }
    }
}
