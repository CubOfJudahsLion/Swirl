//! # Module        :   _Game Builder_
//! - **Purpose**   :   Routines to build game components
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   Tue 08 Mar 2022 02:19:40 PM CST
//! - **Modified**  :   Fri 07/01/2022 10:14 PM


use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, HtmlDivElement, HtmlLabelElement, HtmlImageElement};

#[cfg(debug_assertions)]
use web_sys::console;

use swirl::*;
use swirl::style::shorten::*;

use super::assets::*;
use super::reels::*;


#[allow(dead_code)]
pub struct GameData {
    pub handler         : Rc<GameHandler>,
    pub language        : String,
    pub all_symbols     : String,
    pub initial_reels   : Box<[String]>,
    pub full_reels      : Box<[String]>,
    pub all_coins       : Option<Box<[String]>>,
    pub line_paths      : Box<[String]>,
    pub translations    : HashMap<String, String>,
}


#[derive(Clone)]
pub struct GameHtmlElements {
    pub canvas      : HtmlCanvasElement,
    pub load_div    : HtmlDivElement,
    pub load_label  : HtmlLabelElement,
    pub bkgd_div    : HtmlDivElement,
}

impl GameHtmlElements {
    pub const CANVAS_ID     : &'static str  = "renderTarget";
    pub const LOAD_DIV_ID   : &'static str  = "loaderDiv";
    pub const LOAD_LABEL_ID : &'static str  = "runInfo";
    pub const BKGD_DIV_ID   : &'static str  = "backgroundHolder";

    pub fn new() -> Result<GameHtmlElements, JsValue> {
        Ok(GameHtmlElements {
            canvas      : get_canvas(Self::CANVAS_ID)?,
            load_div    : swirl::get_html_element(Some("div"), Some(Self::LOAD_DIV_ID), &[])?,
            load_label  : swirl::get_label(Self::LOAD_LABEL_ID)?,
            bkgd_div    : swirl::get_html_element(Some("div"), Some(Self::BKGD_DIV_ID), &[])?,
        })
    }
}


#[derive(Clone)]
pub struct GameReels {
    //pub parent  : SceneGraphFrame<f32>,
    pub parent_frm  : SceneGraphFrame<f32>,
    pub parent_node : SceneGraphNode<f32>,
    pub reel_set    : Vec<Reel>,
}

impl GameReels {
    pub const SPACE_BETWEEN : i16   =    0;
    pub const ICON_WIDTH    : u16   =  180;
    pub const ICON_HEIGHT   : u16   =  180;
    pub const Y_START       : i16   =    0;

    //  Creates and populates the reels.
    pub fn new(data: &GameData, assets: &AssetsStruct) -> Result<Self, JsValue> {
        if data.initial_reels.len() != data.full_reels.len() {
            Err(JsValue::from_str("GameReels::new(): Initial reel and full reel arrays have different lengths."))
        }
        else if data.initial_reels.len() == 0 {
            Err(JsValue::from_str("GameReels::new(): reel arrays are empty."))
        }
        else if !data.initial_reels
            .iter()
            .map(|reel| reel.chars().count())
            .fold(
                None as Option<(bool, usize)>,
                |acc, curr| {
                    match acc {
                        Some((false, _))    => acc,
                        Some((true, size))  => Some((curr == size, size)),
                        _                   => Some((true, curr)),
                    }
                }
            )
            .unwrap_or((false, 0))
            .0
        {
            Err(JsValue::from_str(&format!(
                "GameReels::new(): Error: initial reels ({}) .",
                data.initial_reels
                    .iter()
                    .map(|reel| "\"".to_string() + reel + "\"")
                    .fold(
                        "".to_string(),
                        |acc, curr| {
                            if acc == "" {
                                curr.to_string()
                            }
                            else {
                                acc + ", " + &curr
                            }
                        }
                    )
            )))
        }
        else {
            let reel_count: i16 = data.initial_reels
                .len()
                .try_into()
                .map_err(|_| JsValue::from_str("GameReels::new(): Error: too many reels."))?;
            let x_start         =   (   GameVisuals::H_RES 
                                    -   (   Self::ICON_WIDTH as i16
                                        *   reel_count
                                        +   (Self::SPACE_BETWEEN << 2)
                                        )
                                    )
                                >>  1;
            let mut reel_set    =   Vec::with_capacity(reel_count as usize);
            let mut parent_frm  =   SceneGraphFrame::<f32>::new();
            let parent_node     =   SceneGraphNode::Frame(parent_frm.clone());
            parent_frm.set_x_and_y(x_start, Self::Y_START);
            //  Create each column's frame.
            for col in 0 .. reel_count {
                let reel = Reel::new(
                    assets,
                    Self::ICON_HEIGHT,
                    &data.full_reels[col as usize],
                    &data.initial_reels[col as usize]
                )?;
                reel.scene_frame().set_x_and_y(col * (Self::ICON_WIDTH as i16 + Self::SPACE_BETWEEN), 0);
                reel_set.push(reel.clone());
                parent_frm
                    .append_child(reel.generate_scene_node())
                    .map_err(|_| JsValue::from_str("Unable to add reel to reel set."))?;
            }
            Ok(GameReels { parent_frm, parent_node, reel_set, })
        }
    }
}


#[derive(Clone)]
pub struct GameControls {
    pub mouse_driver        : PointerEventDriver<f32>,
    pub spin                : swirl::Button,
    pub spin_events         : PointerEventEmitter<f32>,
    pub spin_node           : SceneGraphNode<f32>,
    pub stop                : swirl::Button,
    pub stop_events         : PointerEventEmitter<f32>,
    pub stop_node           : SceneGraphNode<f32>,
    pub info                : swirl::Button,
    pub info_events         : PointerEventEmitter<f32>,
    pub info_node           : SceneGraphNode<f32>,
    pub coin_plus           : swirl::Button,
    pub coin_plus_events    : PointerEventEmitter<f32>,
    pub coin_plus_node      : SceneGraphNode<f32>,
    pub coin_minus          : swirl::Button,
    pub coin_minus_events   : PointerEventEmitter<f32>,
    pub coin_minus_node     : SceneGraphNode<f32>,
    pub autoplay            : swirl::Button,
    pub autoplay_events     : PointerEventEmitter<f32>,
    pub autoplay_node       : SceneGraphNode<f32>,
}

impl GameControls {
    pub fn new_with_nodes_and_emitter(
        html_elems  : &GameHtmlElements,
        assets      : &AssetsStruct
    )
        -> Result<Self, JsValue>
    {
        let mut mouse_driver    = PointerEventDriver::<f32>::new(html_elems.canvas.clone())?;
        let (     spin
            ,     spin_node
            , mut spin_events
            )                   =
            swirl::Button::new_node_with_logic(
                assets.imgs.interface.button.spin.normal.clone(),
                Some(assets.imgs.interface.button.spin.over.clone()),
                Some(assets.imgs.interface.button.spin.down.clone()),
                ButtonDisabledAppearance::Image(assets.imgs.interface.button.spin.disabled.clone()),
                &mut mouse_driver
            );
        spin_events.set_enabled(true);
        let ( mut stop
            ,     stop_node
            , mut stop_events
            )                   =
            swirl::Button::new_node_with_logic(
                assets.imgs.interface.button.stop.normal.clone(),
                Some(assets.imgs.interface.button.stop.normal.clone()),
                Some(assets.imgs.interface.button.stop.normal.clone()),
                ButtonDisabledAppearance::Absent,
                &mut mouse_driver
            );
        stop_events.set_enabled(true);
        stop.set_button_state(ButtonState::Disabled);
        let ( mut info
            ,     info_node
            , mut info_events
            )                   =
            swirl::Button::new_node_with_logic(
                assets.imgs.interface.button.info.normal.clone(),
                Some(assets.imgs.interface.button.info.normal.clone()),
                Some(assets.imgs.interface.button.info.normal.clone()),
                ButtonDisabledAppearance::Image(assets.imgs.interface.button.info.disabled.clone()),
                &mut mouse_driver
            );
        info_events.set_enabled(false);
        info.set_button_state(ButtonState::Disabled);
        let ( mut coin_minus
            ,     coin_minus_node
            , mut coin_minus_events
            )                   =
            swirl::Button::new_node_with_logic(
                assets.imgs.interface.button.coin_minus.normal.clone(),
                Some(assets.imgs.interface.button.coin_minus.normal.clone()),
                Some(assets.imgs.interface.button.coin_minus.normal.clone()),
                ButtonDisabledAppearance::Image(assets.imgs.interface.button.coin_minus.disabled.clone()),
                &mut mouse_driver
            );
        coin_minus_events.set_enabled(false);
        coin_minus.set_button_state(ButtonState::Disabled);
        let ( mut coin_plus
            ,     coin_plus_node
            , mut coin_plus_events
            )                   =
            swirl::Button::new_node_with_logic(
                assets.imgs.interface.button.coin_plus.normal.clone(),
                Some(assets.imgs.interface.button.coin_plus.normal.clone()),
                Some(assets.imgs.interface.button.coin_plus.normal.clone()),
                ButtonDisabledAppearance::Image(assets.imgs.interface.button.coin_plus.disabled.clone()),
                &mut mouse_driver
            );
        coin_plus_events.set_enabled(false);
        coin_plus.set_button_state(ButtonState::Disabled);
        let (     autoplay
            ,     autoplay_node
            ,     autoplay_events
            )                   =
            swirl::Button::new_node_with_logic(
                assets.imgs.interface.button.autoplay.normal.clone(),
                Some(assets.imgs.interface.button.autoplay.normal.clone()),
                Some(assets.imgs.interface.button.autoplay.normal.clone()),
                ButtonDisabledAppearance::Image(assets.imgs.interface.button.autoplay.disabled.clone()),
                &mut mouse_driver
            );
        Ok( GameControls
            {   mouse_driver
            ,   spin
            ,   spin_events
            ,   spin_node
            ,   stop
            ,   stop_events
            ,   stop_node
            ,   info
            ,   info_events
            ,   info_node
            ,   coin_minus
            ,   coin_minus_events
            ,   coin_minus_node
            ,   coin_plus
            ,   coin_plus_events
            ,   coin_plus_node
            ,   autoplay
            ,   autoplay_events
            ,   autoplay_node
            }
        )
    }
}


#[derive(Clone)]
pub struct GameTextBoxes {
    pub balance     : HtmlImageElement,
    //pub coin_value  : HtmlImageElement,
    pub total_bet   : HtmlImageElement,
    pub total_win   : HtmlImageElement,
}

impl GameTextBoxes {
    pub fn new(assets: &AssetsStruct) -> Self {
        GameTextBoxes {
            balance     : assets.imgs.interface.text_box.balance.clone(),
            //coin_value  : assets.imgs.interface.text_box.coin_value.clone(),
            total_bet   : assets.imgs.interface.text_box.total_bet.clone(),
            total_win   : assets.imgs.interface.text_box.total_win.clone(),
        }
    }
}


#[derive(Clone)]
pub struct GameTexts {
    pub winnings_val    : Text,
    pub total_bet_lbl   : Text,
    pub total_bet_val   : Text,
    pub credits_lbl     : Text,
    pub credits_val     : Text,
}

impl GameTexts {
    pub fn new(data: &GameData, assets: &AssetsStruct) -> Result<Self, JsValue> {
        let winnings_val                    = Text::new(
            "$~~~,~~~.~~",
            FontStyleBuilder::new()
                .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
                .with_size(FontSizeAsUnits(36., Pt))
                .with_fill(Some(Named(White)))
                .build()
        )?;
        let total_bet_lbl                   = Text::new(
            data.translations
                .get("lbl_TotalBet")
                .map(|s| s as &str)
                .unwrap_or_else(|| "~TOTAL BET~"),
            FontStyleBuilder::new()
                .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
                .with_size(FontSizeAsUnits(15., Pt))
                .with_fill(Some(RGB(0xac, 0xe8, 0xfe)))
                .build()
        )?;
        let total_bet_val                   = Text::new(
            "$~,~~~.~~",
            FontStyleBuilder::new()
                .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
                .with_size(FontSizeAsUnits(20., Pt))
                .with_fill(Some(Named(White)))
                .build()
        )?;
        let credits_lbl                     = Text::new(
            data.translations
                .get("lbl_Credits")
                .map(|s| s as &str)
                .unwrap_or_else(|| "~CREDITS~"),
            FontStyleBuilder::new()
                .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
                .with_size(FontSizeAsUnits(15., Pt))
                .with_fill(Some(RGB(0xac, 0xe8, 0xfe)))
                .build()
        )?;
        let credits_val                     = Text::new(
            "~~~,~~~.~~",
            FontStyleBuilder::new()
                .with_family(NamedFamily(assets.fonts.tonic_liver.clone()))
                .with_size(FontSizeAsUnits(20., Pt))
                .with_fill(Some(Named(White)))
                .build()
        )?;
        Ok(GameTexts {
            winnings_val,
            total_bet_lbl,
            total_bet_val,
            credits_lbl,
            credits_val,
        })
    }
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VisualNames {
    Root,
    ReelsAndErasersContainer,
    ErasersContainer,
    ReelsContainer,
    ReelClipperTop,
    ReelClipperBottom,
    ReelsEmptyBox,
    SpaceBetweenReelsAndControls,
    Controls,
    TotalWinBox,
    TotalWinValue,
    TotalWinGroup,
    OtherControlsGroup,
    InfoButton,
    CreditsGroup,
    CreditsBox,
    CreditsTexts,
    CreditsLabel,
    CreditsValue,
    BetValueGroup,
    BetValueBox,
    BetValueControls,
    DecreaseBetButton,
    BetValueTexts,
    BetValueLabel,
    BetValue,
    IncreaseBetButton,
    SpinButton,
    StopButton,
    AutoplayButton,
}

impl Display for VisualNames {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}


pub struct LayoutDimRatio {
    pub ratio   : f32,
    pub dims    : Dimensions<i16>,
    pub layout  : Box<dyn Layout<VisualNames>>,
}

impl Clone for LayoutDimRatio {
    fn clone(&self) -> Self {
        LayoutDimRatio {
            ratio   : self.ratio,
            dims    : self.dims.clone(),
            layout  : self.layout.deep_clone(),
        }
    }
}

pub struct GameLayouts(pub Vec<LayoutDimRatio>);

impl Clone for GameLayouts {
    fn clone(&self) -> Self {
        GameLayouts(
            self.0.iter().map(|ldr| ldr.clone()).collect()
        )
    }
}


#[derive(Clone)]
pub struct GameVisuals {
    pub renderer    : Renderer,
    pub buttons     : GameControls,
    pub text_boxes  : GameTextBoxes,
    pub texts       : GameTexts,
    pub reels       : GameReels,
    pub layouts     : GameLayouts,
}


impl GameVisuals {
    pub const H_RES : i16   = 1016;
    pub const V_RES : i16   =  715;
    pub const FPS   : i32   =   50;
    pub const MSPF  : i32   = 1000 / Self::FPS;

    fn make_layouts(
        reels       : &mut GameReels,
        buttons     : &mut GameControls,
        text_boxes  : &mut GameTextBoxes,
        texts       : &mut GameTexts
    )
        -> Result<GameLayouts, LayoutError<VisualNames>>
    {
        // First, let's create the objects common to all layouts --
        // the base building blocks for all of them.
        let reel_count                  = reels.reel_set.len() as i16;
        let full_reels_width            = GameReels::ICON_WIDTH as i16 * reel_count + GameReels::SPACE_BETWEEN * (reel_count - 1);
        let reel_clipper_top            = EraserRect::new(&Rect::<i16>::new_by_dim(0, 0, GameReels::ICON_HEIGHT as i16, full_reels_width));
        let reel_clipper_bottom         = EraserRect::new(&Rect::<i16>::new_by_dim(0, 0, GameReels::ICON_HEIGHT as i16, full_reels_width));

        use VisualNames::*;

        let info_button                 = SingleObject::new(
            Some(InfoButton),
            Padding::Directional(Dimensions {width: 5, height: 0}),
            HorizontalAlign::Left,
            VerticalAlign::Bottom,
            LayoutScaling::NoScaling,
            buttons.info_node.clone()
        );

        let autoplay_button             = SingleObject::new(
            Some(AutoplayButton),
            Padding::Directional(Dimensions {width: 5, height: 0}),
            HorizontalAlign::Right,
            VerticalAlign::Bottom,
            LayoutScaling::NoScaling,
            buttons.autoplay_node.clone()
        );

        let spin_button                 = SingleObject::new(
            Some(SpinButton),
            Padding::EachEdge(Rect {top: 20, left: 5, bottom: 0, right: 0}),
            HorizontalAlign::Right,
            VerticalAlign::Bottom,
            LayoutScaling::NoScaling,
            buttons.spin_node.clone()
        );

        let clipped_reels               = Superposition::new(
            Some(ReelsAndErasersContainer),
            Padding::Directional(Dimensions {
                width   :  0,
                height  : -(GameReels::ICON_HEIGHT as i16)
            }),
            HorizontalAlign::Center,
            VerticalAlign::Bottom,
            LayoutScaling::NoScaling
        )
            .with(
                &SingleObject::new(
                    Some(ReelsContainer),
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling,
                    reels.parent_node.clone()
                )
            )
            .and_then(|layout| layout.with(
                &(Column::new(
                    Some(ErasersContainer),
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling
                )
                    .with(
                        HeightConstraints::MinHeight,
                        0,
                        &SingleObject::new_from_drawable(
                            Some(ReelClipperTop),
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Top,
                            LayoutScaling::NoScaling,
                            &reel_clipper_top
                        ).0
                    )
                    .and_then(|layout| layout.with(
                        HeightConstraints::MaxHeight,
                        0,
                        &SingleObject::new_from_drawable(
                            Some(ReelsEmptyBox),
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                            &EmptyBox::new(
                                reels.parent_frm.width()  as i16,
                                reels.parent_frm.height() as i16
                            )
                        ).0
                    ))
                    .and_then(|layout| layout.with(
                        HeightConstraints::MinHeight,
                        0,
                        &SingleObject::new_from_drawable(
                            Some(ReelClipperBottom),
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Bottom,
                            LayoutScaling::NoScaling,
                            &reel_clipper_bottom
                        ).0
                    ))?
                )
            ))?;

        let total_win_group             = Superposition::new(
            Some(TotalWinGroup),
            Padding::NoPadding,
            HorizontalAlign::Center,
            VerticalAlign::Top,
            LayoutScaling::NoScaling,
         )
            .with(
                &SingleObject::new_from_drawable(
                    Some(TotalWinBox),
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling,
                    &Sprite::new(text_boxes.total_win.clone())
                ).0
            )
            .and_then(|layout| layout.with(
                &SingleObject::new_from_text(
                    Some(TotalWinValue),
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling,
                    &texts.winnings_val
                ).0
            ))?;

        let credits_group               = Superposition::new(
            Some(VisualNames::CreditsGroup),
            Padding::Directional(Dimensions {width: 5, height: 0}),
            HorizontalAlign::Center,
            VerticalAlign::Bottom,
            LayoutScaling::NoScaling,
        )
            .with(
                &SingleObject::new_from_drawable(
                    Some(VisualNames::CreditsBox),
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling,
                    &Sprite::new(text_boxes.balance.clone())
                ).0
            )
            .and_then(|layout| layout.with(
                &(Column::new(
                    Some(VisualNames::CreditsTexts),
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling,
                )
                    .with(
                        HeightConstraints::MinHeight,
                        0,
                        &SingleObject::new_from_text(
                            Some(VisualNames::CreditsLabel),
                            Padding::EachEdge(Rect {top: 0, left: 0, bottom: 6, right: 0}),
                            HorizontalAlign::Center,
                            VerticalAlign::Top,
                            LayoutScaling::NoScaling,
                            &texts.credits_lbl
                        ).0
                    )
                    .and_then(|label| label.with(
                        HeightConstraints::MinHeight,
                        0,
                        &SingleObject::new_from_text(
                            Some(VisualNames::CreditsValue),
                            Padding::EachEdge(Rect {top: 6, left: 0, bottom: 0, right: 0}),
                            HorizontalAlign::Center,
                            VerticalAlign::Bottom,
                            LayoutScaling::NoScaling,
                            &texts.credits_val
                        ).0
                    ))?
                )
            ))?;

        let bet_value_group             = Superposition::new(
            Some(BetValueGroup),
            Padding::Directional(Dimensions {width: 5, height: 0}),
            HorizontalAlign::Center,
            VerticalAlign::Bottom,
            LayoutScaling::NoScaling,
        )
            .with(
                &SingleObject::new_from_drawable(
                    Some(BetValueBox),
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling,
                    &Sprite::new(text_boxes.total_bet.clone())
                ).0
            )
            .and_then(|layout| layout.with(
                &(Row::new(
                    Some(BetValueControls),
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Bottom,
                    LayoutScaling::NoScaling,
                )
                    .with(
                        WidthConstraints::MinWidth,
                        0,
                        &SingleObject::new(
                            Some(DecreaseBetButton),
                            Padding::NoPadding,
                            HorizontalAlign::Left,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                            buttons.coin_minus_node.clone()
                        )
                    )
                    .and_then(|layout| layout.with(
                        WidthConstraints::MaxWidth,
                        0,
                        &(Column::new(
                            Some(BetValueTexts),
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                        )
                            .with(
                                HeightConstraints::MinHeight,
                                0,
                                &SingleObject::new_from_text(
                                    Some(BetValueLabel),
                                    Padding::Uniform(2),
                                    HorizontalAlign::Center,
                                    VerticalAlign::Bottom,
                                    LayoutScaling::NoScaling,
                                    &texts.total_bet_lbl
                                ).0
                            )
                            .and_then(|layout| layout.with(
                                HeightConstraints::MinHeight,
                                0,
                                &SingleObject::new_from_text(
                                    Some(BetValue),
                                    Padding::Uniform(2),
                                    HorizontalAlign::Center,
                                    VerticalAlign::Top,
                                    LayoutScaling::NoScaling,
                                    &texts.total_bet_val
                                ).0
                            ))?
                        )
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MinWidth,
                        0,
                        &SingleObject::new(
                            Some(IncreaseBetButton),
                            Padding::NoPadding,
                            HorizontalAlign::Right,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                            buttons.coin_plus_node.clone()
                        )
                    ))?
                )
            ))?;

        #[allow(unused_variables)]
        let desktop                     = Column::<VisualNames>::new(
            Some(VisualNames::Root),
            Padding::NoPadding,
            HorizontalAlign::Center,
            VerticalAlign::Middle,
            LayoutScaling::NoScaling,
        )
            .with(
                HeightConstraints::MinHeight,
                0,
                &clipped_reels
            )
            .and_then(|layout| layout .with(
                HeightConstraints::MinHeight,
                2,
                &SingleObject::new_from_drawable(
                    Some(SpaceBetweenReelsAndControls),
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling,
                    &EmptyBox::new(2, 32)
                ).0
            ))
            .and_then(|layout| layout.with(
                HeightConstraints::MinHeight,
                1,
                &(Superposition::new(
                    Some(Controls),
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Top,
                    LayoutScaling::NoScaling,
                )
                    .with(&total_win_group)
                    .and_then(|layout| layout.with(
                        &(Row::new(
                            Some(VisualNames::OtherControlsGroup),
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Bottom,
                            LayoutScaling::NoScaling
                        )
                            .with(
                                WidthConstraints::MinWidth,
                                0,
                                &info_button
                            )
                            .and_then(|layout| layout.with(
                                WidthConstraints::MinWidth,
                                0,
                                &credits_group
                            ))
                            .and_then(|layout| layout.with(
                                WidthConstraints::MinWidth,
                                0,
                                &bet_value_group
                            ))
                            .and_then(|layout| layout.with(
                                WidthConstraints::MinWidth,
                                0,
                                &autoplay_button
                            ))
                            .and_then(|layout| layout.with(
                                WidthConstraints::MinWidth,
                                0,
                                &spin_button
                            ))?
                        )
                    ))?
                )
            ))?;

        const VMOB_REELS_SCALE: f32     =   0.467;
        let winnings_reels_width_box    =   ColorBox::new_from_dims(
            &Dimensions {
                width   : (reels.parent_frm.parent_width() * VMOB_REELS_SCALE).ceil() as i16,
                height  : text_boxes.total_win.height() as i16,
            },
            Color::RGB(0x01, 0x00, 0x22)
        );
        let pixel_spacer                =   || SingleObject::new_from_drawable(
            None,
            Padding::NoPadding,
            HorizontalAlign::Center,
            VerticalAlign::Middle,
            LayoutScaling::NoScaling,
            &EmptyBox::new(2, 2)
        ).0;
        const VERT_REEL_PAD: i16        =   15;
        let full_width_line             =   ColorBox::new_from_dims(
            &Dimensions {
                width   :   (reels.parent_frm.parent_width() * VMOB_REELS_SCALE).ceil() as i16
                        +   (VERT_REEL_PAD << 1),
                height  :   1,
            },
            Color::RGB(0x01, 0x00, 0x22)
        );
        let full_third                  =   (   (   (reels.parent_frm.parent_width() * VMOB_REELS_SCALE)
                                                +   (VERT_REEL_PAD as f32 * 2.)
                                                )
                                            /   3.
                                            ).round() as i16;
        let full_third_box              =   ColorBox::new_from_dims(
            &Dimensions {
                width   : full_third,
                height  : text_boxes.total_win.height() as i16,
            },
            Color::RGB(0x01, 0x00, 0x22)
        );

        let mut smaller_reels           =   clipped_reels.clone();
        smaller_reels.set_scale(LayoutScaling::UniformScaling(VMOB_REELS_SCALE));
        smaller_reels.set_padding(Padding::EachEdge(Rect {
            top     : smaller_reels.padding().top(),
            left    : smaller_reels.padding().left(),
            right   : smaller_reels.padding().right(),
            bottom  : smaller_reels.padding().bottom() + 16,
        }));

        #[allow(unused_variables)]
        let mobile_vertical             =   Column::<VisualNames>::new(
            None,
            Padding::NoPadding,
            HorizontalAlign::Center,
            VerticalAlign::Bottom,
            LayoutScaling::NoScaling
        )
            .with(
                HeightConstraints::MinHeight,
                0,
                &SingleObject::new_from_drawable(
                    None,
                    Padding::EachEdge(Rect {
                        top     : 0,
                        left    : 0,
                        bottom  : (reels.parent_frm.parent_height() as f32 * VMOB_REELS_SCALE * 3. / 4.).ceil() as i16,
                        right   : 0,
                    }),
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling,
                    &EmptyBox::new(2, 2)
                ).0
            )
            .and_then(|layout| layout.with(
                HeightConstraints::MinHeight,
                0,
                &smaller_reels
            ))
            .and_then(|layout| layout.with(
                HeightConstraints::MinHeight,
                1,
                &(Superposition::new(
                    None,
                    Padding::EachEdge(Rect {
                        top     :  0,
                        left    :  0,
                        bottom  : 12,
                        right   :  0,
                    }),
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling
                )
                    .with(
                        &SingleObject::new_from_drawable(
                            None,
                            Padding::Directional(Dimensions {
                                width   : VERT_REEL_PAD,
                                height  : 0,
                            }),
                            HorizontalAlign::Center,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                            &winnings_reels_width_box
                        ).0
                    )
                    .and_then(|layout| layout.with(
                        &SingleObject::new_from_text(
                            None,
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                            &texts.winnings_val
                        ).0
                    ))?
                )
            ))
            .and_then(|layout| layout.with(
                HeightConstraints::MinHeight,
                1,
                &(Row::new(
                    None,
                    Padding::EachEdge(Rect {
                        top     :  0,
                        left    :  0,
                        bottom  : 12,
                        right   :  0,
                    }),
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling,
                )
                    .with(
                        WidthConstraints::MaxWidth,
                        1,
                        &pixel_spacer()
                    )
                    .and_then(|layout| layout.with(
                        WidthConstraints::MinWidth,
                        0,
                        &SingleObject::new(
                            None,
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                            buttons.info_node.clone()
                        )
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MaxWidth,
                        1,
                        &pixel_spacer()
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MinWidth,
                        0,
                        &SingleObject::new(
                            None,
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                            buttons.autoplay_node.clone()
                        )
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MaxWidth,
                        1,
                        &pixel_spacer()
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MinWidth,
                        0,
                        &SingleObject::new(
                            None,
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                            buttons.spin_node.clone()
                        )
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MaxWidth,
                        1,
                        &pixel_spacer()
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MinWidth,
                        0,
                        &SingleObject::new(
                            None,
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                            buttons.coin_minus_node.clone()
                        )
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MaxWidth,
                        1,
                        &pixel_spacer()
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MinWidth,
                        0,
                        &SingleObject::new(
                            None,
                            Padding::NoPadding,
                            HorizontalAlign::Center,
                            VerticalAlign::Middle,
                            LayoutScaling::NoScaling,
                            buttons.coin_plus_node.clone()
                        )
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MaxWidth,
                        1,
                        &pixel_spacer()
                    ))?
               )
            ))
            .and_then(|layout| layout.with(
                HeightConstraints::MinHeight,
                1,
                &SingleObject::new_from_drawable(
                    None,
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Middle,
                    LayoutScaling::NoScaling,
                    &full_width_line
                ).0
            ))
            .and_then(|layout| layout.with(
                HeightConstraints::MinHeight,
                1,
                &(Row::new(
                    None,
                    Padding::NoPadding,
                    HorizontalAlign::Center,
                    VerticalAlign::Top,
                    LayoutScaling::NoScaling
                )
                    .with(
                        WidthConstraints::MinWidth,
                        0,
                        &(Superposition::new(
                            None,
                            Padding::NoPadding,
                            HorizontalAlign::Left,
                            VerticalAlign::Top,
                            LayoutScaling::NoScaling
                        )
                            .with(
                                &SingleObject::new_from_drawable(
                                    None,
                                    Padding::NoPadding,
                                    HorizontalAlign::Center,
                                    VerticalAlign::Middle,
                                    LayoutScaling::NoScaling,
                                    &full_third_box
                                ).0
                            )
                            .and_then(|layout| layout.with(
                                &(Column::new(
                                    None,
                                    Padding::NoPadding,
                                    HorizontalAlign::Center,
                                    VerticalAlign::Middle,
                                    LayoutScaling::NoScaling
                                )
                                    .with(
                                        HeightConstraints::MinHeight,
                                        0,
                                        &SingleObject::new_from_text(
                                            None,
                                            Padding::EachEdge(Rect {
                                                top     :  0,
                                                left    :  0,
                                                bottom  :  5,
                                                right   :  0,
                                            }),
                                            HorizontalAlign::Center,
                                            VerticalAlign::Bottom,
                                            LayoutScaling::NoScaling,
                                            &texts.credits_lbl
                                        ).0
                                    )  
                                    .and_then(|layout| layout.with(
                                        HeightConstraints::MinHeight,
                                        0,
                                        &SingleObject::new_from_text(
                                            None,
                                            Padding::NoPadding,
                                            HorizontalAlign::Center,
                                            VerticalAlign::Top,
                                            LayoutScaling::NoScaling,
                                            &texts.credits_val
                                        ).0
                                    ))?
                                )
                            ))?
                        )
                    )
                    .and_then(|layout| layout.with(
                        WidthConstraints::MaxWidth,
                        0,
                        &pixel_spacer()
                    ))
                    .and_then(|layout| layout.with(
                        WidthConstraints::MinWidth,
                        0,
                        &(Superposition::new(
                            None,
                            Padding::NoPadding,
                            HorizontalAlign::Right,
                            VerticalAlign::Top,
                            LayoutScaling::NoScaling
                        )
                            .with(
                                &SingleObject::new_from_drawable(
                                    None,
                                    Padding::NoPadding,
                                    HorizontalAlign::Center,
                                    VerticalAlign::Middle,
                                    LayoutScaling::NoScaling,
                                    &full_third_box
                                ).0
                            )
                            .and_then(|layout| layout.with(
                                &(Column::new(
                                    None,
                                    Padding::NoPadding,
                                    HorizontalAlign::Center,
                                    VerticalAlign::Middle,
                                    LayoutScaling::NoScaling
                                )
                                    .with(
                                        HeightConstraints::MinHeight,
                                        0,
                                        &SingleObject::new_from_text(
                                            None,
                                            Padding::EachEdge(Rect {
                                                top     :  0,
                                                left    :  0,
                                                bottom  :  5,
                                                right   :  0,
                                            }),
                                            HorizontalAlign::Center,
                                            VerticalAlign::Bottom,
                                            LayoutScaling::NoScaling,
                                            &texts.total_bet_lbl
                                        ).0
                                    )
                                    .and_then(|layout| layout.with(
                                        HeightConstraints::MinHeight,
                                        0,
                                        &SingleObject::new_from_text(
                                            None,
                                            Padding::NoPadding,
                                            HorizontalAlign::Center,
                                            VerticalAlign::Top,
                                            LayoutScaling::NoScaling,
                                            &texts.total_bet_val
                                        ).0
                                    ))?
                                )
                            ))?
                        )
                    ))?
                )
            ))?;

        const BIGGER_REEL_SCALE : f32   = 0.9;
        let mut bigger_reels            = clipped_reels.clone();
        bigger_reels.set_scale(LayoutScaling::UniformScaling(BIGGER_REEL_SCALE));
        let bigger_reel_width           = reels.parent_node.parent_width() as f32 * BIGGER_REEL_SCALE;
        const SEP_SIZE          : i16   = 12;
        let under_reel_seps             = bigger_reel_width - (SEP_SIZE << 1) as f32;
        let color_rect_narrow           = swirl::ColorBox::new_from_dims(
            &Dimensions {
                width   : (under_reel_seps * 2. / 9.) as i16,
                height  : text_boxes.total_win.height() as i16,
            },
            Color::RGB(0x01, 0x00, 0x22)
        );
        let color_rect_wide             = swirl::ColorBox::new_from_dims(
            &Dimensions {
                width   : (under_reel_seps * 5. / 9.) as i16,
                height  : text_boxes.total_win.height() as i16,
            },
            Color::RGB(0x01, 0x00, 0x22)
        );

        //  This one, for an useful comparison, is done with the `layoutm` macro.
        let left_buttons =  layoutm! {
            /*(min, 0)*/ col (pad(0, 0, 0, SEP_SIZE)) {
                (max, 1) single (draw(EmptyBox::new(1, 1))),
                (min, 0) single (s(1.25) node(buttons.info_node)),
                (max, 1) single (draw(EmptyBox::new(1, 1))),
                (min, 0) single (pad(0, 0, 6, 0) s(1.25) node(buttons.coin_plus_node)),
                (min, 0) single (s(1.25) node(buttons.coin_minus_node)),
                (max, 1) single (draw(EmptyBox::new(1, 1)))
            }
        }?;

        let reels_and_values = layoutm! {
            /*(min, 0)*/ col () {
                (min, 0) lay (bigger_reels),
                (min, 0) row (pad(SEP_SIZE, 0, 0, 0)) {
                    (parts(2), 0) superpos () {
                        single (draw(color_rect_narrow)),
                        col () {
                            (min, 0) single (pad(0, 0, SEP_SIZE >> 2, 0) v(b) text(texts.total_bet_lbl)),
                            (min, 0) single (v(t) text(texts.total_bet_val))
                        }
                    },
                    (parts(5), 0) superpos (pad(6, 0)) {
                        single (draw(color_rect_wide)),
                        single (pad(SEP_SIZE, 0) text(texts.winnings_val))
                    },
                    (parts(2), 0) superpos () {
                        single (draw(color_rect_narrow)),
                        col () {
                            (min, 0) single (pad(0, 0, SEP_SIZE, 0) v(b) text(texts.credits_lbl)),
                            (min, 0) single (v(t) text(texts.credits_val))
                        }
                    }
                }
            }
        }?;

        let right_buttons = layoutm! {
            /*(min, 0)*/ col (pad(0, SEP_SIZE, 0, 0)) {
                (max, 1) single (draw(EmptyBox::new(1, 1))),
                (min, 0) single (s(1.25) node(buttons.autoplay_node)),
                (max, 1) single (draw(EmptyBox::new(1, 1))),
                (min, 0) single (s(1.25) node(buttons.spin_node)),
                (max, 1) single (draw(EmptyBox::new(1, 1)))
            }
        }?;

        #[allow(unused_variables)]
        let mobile_horizontal = layoutm! {
            row () {
                (min, 1) lay (left_buttons),
                (min, 0) lay (reels_and_values),
                (min, 1) lay (right_buttons)
            }
        }?;

        let mut layouts: Vec<Box<dyn Layout<VisualNames>>> = vec![
            Box::new(mobile_vertical),
            Box::new(mobile_horizontal)
        ];
        if !is_mobile() {
            layouts.push(Box::new(desktop));
        }
        let mut lay_dim_ratios = layouts
            .drain(..)
            .map(|layout| {
                let dims = layout.calc_naive_bounds();
                LayoutDimRatio {
                    ratio: dims.width as f32 / dims.height as f32,
                    dims,
                    layout,
                }
            })
            .collect::<Vec<_>>();
        lay_dim_ratios
            .iter()
            .for_each(|LayoutDimRatio {ratio, dims, ..}| {
                #[cfg(debug_assertions)]
                console::log_1(&JsValue::from_str(&format!(
                    "Layout demands: {:?}, ratio {:?}",
                    dims, ratio
                )));
            });
        lay_dim_ratios.sort_by(|v1, v2| f32::partial_cmp(&v1.ratio, &v2.ratio).unwrap());
        Ok(GameLayouts(lay_dim_ratios))
    }

    pub fn new(data: &GameData, assets: &AssetsStruct, html_elems: &GameHtmlElements) -> Result<Self, JsValue> {
        //  Create shared resources.
        let renderer                    = Renderer::new(&html_elems.canvas)?;
        //  Start with the assets.
        //  First, the reels proper.
        let mut reels                   = GameReels::new(data, assets)?;
        //  Buttons
        let mut buttons                 = GameControls::new_with_nodes_and_emitter(html_elems, assets)?;
        buttons.spin_events.subscribe(
            PointerEventType::Click,
            {
                let handler = data.handler.clone();
                move |_| {
                    handler.on_spin_clicked();
                    ActionAfterEvent::NoAction
                }
            }
        );
        buttons.stop_events.subscribe(
            PointerEventType::Click,
            {
                let handler = data.handler.clone();
                move |_| {
                    handler.on_stop_clicked();
                    ActionAfterEvent::NoAction
                }
            }
        );
        buttons.coin_minus_events.subscribe(
            PointerEventType::Click,
            {
                let handler = data.handler.clone();
                move |_| {
                    handler.on_total_bet_clicked(ChangeAction::<f64>::Decrease, Some(false));
                    ActionAfterEvent::NoAction
                }
            }
        );
        buttons.coin_plus_events.subscribe(
            PointerEventType::Click,
            {
                let handler = data.handler.clone();
                move |_| {
                    handler.on_total_bet_clicked(ChangeAction::<f64>::Increase, Some(false));
                    ActionAfterEvent::NoAction
                }
            }
        );
        let mut texts                   = GameTexts::new(data, assets)?;
        let mut text_boxes              = GameTextBoxes::new(assets);
        //
        //  Auto-generate a layout.
        let layouts                     = Self::make_layouts(&mut reels, &mut buttons, &mut text_boxes, &mut texts)
            .map_err(|err| JsValue::from(&format!("{}", err)))?;
        //let layout_dims                 = layout.calc_naive_bounds().clone();
        //#[cfg(debug_assertions)]
        //console::log_1(&JsValue::from_str(&format!("Layout requires {}x{} pixels.", layout_dims.width, layout_dims.height)));
        //let scene_left                  = Self::H_RES - layout_dims.width  >> 1;
        ////let scene_top                   = Self::V_RES - layout_dims.height >> 1;
        //let scene_top                   = 0i16;
        //let GraphCalcOutput{
        //    graph       : scene_node,
        //    .. //mut named_nodes,
        //}                               =
        //    (if is_mobile() {
        //        layouts.mobile_vertical
        //    }
        //     else {
        //         layouts.desktop
        //    }).calc_graph(
        //        &Rect {
        //            top     : scene_top,
        //            left    : scene_left,
        //            //bottom  : scene_top  + layout_dims.height - 1,
        //            bottom  : Self::V_RES - 1,
        //            right   : scene_left + layout_dims.width  - 1,
        //        },
        //        None
        //    )
        //    .map_err(|err| JsValue::from(&format!("{}", err)))?;
        //#[cfg(debug_assertions)]
        //{
        //    //  Now we will rip apart the layout and rebuild it to see
        //    //  the `fast_forward` mechanism in action.
        //    fn unravel_hierarchy(named_nodes: &mut HashMap<VisualNames, SceneGraphNode<f32>>) {
        //        for (_, node) in named_nodes.iter_mut() {
        //            let mut node_copy = node.clone();
        //            if let &mut Some(ref mut par) = node.parent_mut() {
        //                let _ = par.remove_child(&mut node_copy);
        //            }
        //            node.set_x_and_y(0, 0);
        //        }
        //    }
        //    console::log_1(&JsValue::from(&format!(
        //        "Number of children in main scene: {}",
        //        scene_node.run_either(|_| 0, |frame| frame.children_count())
        //    )));
        //    console::log_1(&JsValue::from("Destroying layout-calculated hierarchy."));
        //    unravel_hierarchy(&mut named_nodes);
        //    //  And now, we remake it automagically.
        //    console::log_1(&JsValue::from("Fast-forwarding (recalculating) hierarchy."));
        //    layout.fast_forward().expect("Fast-forwarding incomplete.");
        //    console::log_1(&JsValue::from(&format!(
        //        "Number of children in main scene: {}",
        //        scene_node.run_either(|_| 0, |frame| frame.children_count())
        //    )));
        //}
        //  Automate rendering.
        //renderer.set_auto_rendering(Some(Self::MSPF as i32), &scene_node, BeforeRendering::Clear)
        //    .map_err(|_| JsValue::from_str("Cannot start auto-renderer."))?;
        // Now gather renderer, scene and all interactive assets and return them.
        Ok(GameVisuals {
            renderer,
            buttons,
            texts,
            text_boxes,
            reels,
            layouts,
        })
    }
}
