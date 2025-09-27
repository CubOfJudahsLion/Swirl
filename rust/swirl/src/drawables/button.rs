//! # Module        :   Button
//! - **Purpose**   :   A standard clickable button with default, hovering and down appearances provided
//!                     by images. An optional "disabled" look can be provided by an optional image, by
//!                     showing the default appearance dimmed down through an alpha factor, or simply
//!                     by staying invisible.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Thu Apr  8 11:57:43 AM CST 2021
//! - **Modified**  :   Mon 05/23/2022 03:33 PM


use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::utils::Shared;
use crate::events::interactive::pointer::driver::PointerEventDriver;
use crate::scene_graph::leaf::SceneGraphLeaf;
use crate::scene_graph::node::SceneGraphNode;
use super::super::*;


#[derive(Clone, Debug)]
pub enum ButtonDisabledAppearance {
    Image(HtmlImageElement),
    Alpha(f64),
    Absent,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonState {
    Default,
    Hover,
    Down,
    Disabled,
}


#[derive(Clone, Debug)]
struct ButtonInner {
    default         : HtmlImageElement,
    hover           : Option<HtmlImageElement>,
    down            : Option<HtmlImageElement>,
    disabled_img    : Option<HtmlImageElement>,
    disabled_alpha  : Option<f64>,
    curr_img        : Option<HtmlImageElement>,
    curr_alpha      : Option<f64>,
    bounds          : Rect<i16>,
    state           : ButtonState,
    visible         : bool,
    alpha           : f64,
    mode            : BlendMode,
}

#[derive(Debug)]
pub struct Button(Shared<ButtonInner>);

impl Clone for Button {
    fn clone(&self) -> Self {
        Button(self.0.clone())
    }
}

impl Button {
    pub fn new(
        default : HtmlImageElement,
        hover   : Option<HtmlImageElement>,
        down    : Option<HtmlImageElement>,
        disabled: ButtonDisabledAppearance
    ) -> Self
    {
        let curr_img    = Some(default.clone());
        let curr_alpha  = None;
        //let hover       = hover.unwrap_or_else(|| default.clone());
        //let down        = down.unwrap_or_else(|| default.clone());
        use ButtonDisabledAppearance::*;
        let (disabled_alpha, disabled_img) =
            match disabled {
                Image(img)      => (None, Some(img)),
                Alpha(alpha)    => (Some(alpha), Some(default.clone())),
                Absent          => (None, None),
            };
        let bounds      = [
            &hover,
            &down,
            &disabled_img
        ]
            .iter()
            .filter_map(|img| { img.as_ref().map(|img|
                Rect::new_by_dim(0, 0, img.height() as i16, img.width() as i16)
            )})
            .fold(
                Rect::new_by_dim(0, 0, default.height() as i16, default.width() as i16),
                |all, curr| all.include(&curr)
            );
        Button(Shared::new(ButtonInner {
            default,
            hover,
            down,
            disabled_img,
            disabled_alpha,
            curr_img,
            curr_alpha,
            bounds,
            state   : ButtonState::Default,
            visible : true,
            alpha   : 1.,
            mode    : BlendMode::Normal,
        }))
    }

    pub fn button_state(&self) -> ButtonState { self.0.state }

    pub fn set_button_state(&mut self, new_state: ButtonState) {
        self.0.state = new_state;
        match self.0.state {
            ButtonState::Default    => {
                self.0.curr_img     = Some(self.0.default.clone());
                self.0.curr_alpha   = None;
            },
            ButtonState::Disabled   => {
                self.0.curr_img     = self.0.disabled_img.clone();
                self.0.curr_alpha   = self.0.disabled_alpha;
            },
            ButtonState::Down       => {
                self.0.curr_img     = self.0.down.clone().or(Some(self.0.default.clone()));
                self.0.curr_alpha   = None;
            },
            ButtonState::Hover      => {
                self.0.curr_img     = self.0.hover.clone().or(Some(self.0.default.clone()));
                self.0.curr_alpha   = None;
            },
        }
    }

    pub fn new_node_with_logic(
        default : HtmlImageElement,
        hover   : Option<HtmlImageElement>,
        down    : Option<HtmlImageElement>,
        disabled: ButtonDisabledAppearance,
        driver  : &mut PointerEventDriver<f32>
    )
        -> (Button, SceneGraphNode<f32>, PointerEventEmitter<f32>)
    {
        let button          = Self::new(default, hover, down, disabled);
        let node            = SceneGraphLeaf::new_node(&button);
        let mut raw_events  = driver.add_emitter(&node);
        let outer_events    = PointerEventEmitter::<f32>::new(&node);
        use PointerEventType::*;
        [Move, Click, DblClick]
            .iter()
            .for_each(|evt| {
                let mut outer_events    = outer_events.clone();
                let button              = button.clone();
                raw_events.subscribe(*evt, move |mev: &PointerEvent<f32>| {
                    if button.button_state() != ButtonState::Disabled {
                        outer_events.emit(mev);
                    }
                    ActionAfterEvent::NoAction
                });
            });
        {
            let mut outer_events    = outer_events.clone();
            let mut button          = button.clone();
            raw_events.subscribe(PointerEventType::Enter, move |mev: &PointerEvent<f32>| {
                if button.button_state() != ButtonState::Disabled {
                    button.set_button_state(ButtonState::Hover);
                    outer_events.emit(mev);
                }
                ActionAfterEvent::NoAction
            });
        }
        {
            let mut outer_events    = outer_events.clone();
            let mut button          = button.clone();
            raw_events.subscribe(PointerEventType::Leave, move |mev: &PointerEvent<f32>| {
                if button.button_state() != ButtonState::Disabled {
                    button.set_button_state(ButtonState::Default);
                    outer_events.emit(mev);
                }
                ActionAfterEvent::NoAction
            });
        }
        {
            let mut outer_events    = outer_events.clone();
            let mut button          = button.clone();
            raw_events.subscribe(PointerEventType::Down, move |mev: &PointerEvent<f32>| {
                if button.button_state() != ButtonState::Disabled {
                    button.set_button_state(ButtonState::Down);
                    outer_events.emit(mev);
                }
                ActionAfterEvent::NoAction
            });
        }
        {
            let mut outer_events    = outer_events.clone();
            let mut button          = button.clone();
            raw_events.subscribe(PointerEventType::Up, move |mev: &PointerEvent<f32>| {
                if button.button_state() != ButtonState::Disabled {
                    button.set_button_state(ButtonState::Hover);
                    outer_events.emit(mev);
                }
                ActionAfterEvent::NoAction
            });
        }
        (button, node, outer_events)
    }
}

impl Bounded<i16> for Button {
    #[inline(always)]
    fn bounds(&self) -> Option<Rect<i16>> {
        Some(self.0.bounds.clone())
    }
}


impl Drawable for Button {
    fn drawable(&self) -> bool {
        self.0.visible
    }

    fn set_drawable(&mut self, drawable: bool) {
        self.0.visible = drawable;
    }

    /// The final alpha is the product of the value set through `set_alpha`
    /// and the opacity optionally set by disabled mode.
    fn alpha(&self) -> f64 { self.0.alpha * self.0.curr_alpha.unwrap_or(1.) }

    fn set_alpha(&mut self, alpha: f64) {
        self.0.alpha =
            if alpha < 0. {
                0.
            }
            else if alpha > 1. {
                1.
            }
            else {
                alpha
            };
    }

    fn blend_mode(&self) -> BlendMode { self.0.mode }

    fn set_blend_mode(&mut self, blend_mode: BlendMode) { self.0.mode = blend_mode; }

    fn draw(
        &self,
        context : &CanvasRenderingContext2d
    ) -> Result<(), JsValue>
    {
        if let &Some(ref img) = &self.0.curr_img {
            context.draw_image_with_html_image_element_and_dw_and_dh(
                img,
                0.0,
                0.0,
                img.width()  as f64,
                img.height() as f64
            )
        }
        else {
            Ok(())
        }
    }
}
