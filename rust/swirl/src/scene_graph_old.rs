//! Module  :   Scene Graph
//! Purpose :   A set of stackable containers for Drawables.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Mar 16 08:20:07 PM CST 2021
//! Modified:   Thu Jul  8 03:27:46 PM CST 2021


use std::cmp::{min, max};

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::base_traits::{Positioned, Bounded, Drawable};
use crate::utils::shared::*;
use crate::math::geom::{Point, Rect};


/// A member of this type class needs to have a position and bounds, and
/// then to have the capacity to be inside a scene graph (i.e., to have
/// a `parent`.
pub trait SceneGraph<N>: Positioned<N> + Bounded<N> {
    fn parent(&self) -> Option<&SceneGraphFrame>;

    fn global_bounds(&self) -> Rect<N> {
        let mut bounds      = self.local_bounds();
        let mut next_parent = self.parent();
        while let Some(par) = next_parent {
            bounds      = bounds + Point::new(par.x(), par.y());
            next_parent = par.parent();
        }
        bounds
    }
}

pub(crate) struct SceneGraphLeafInner {
    pub(crate) pos      : Point<i32>,
    pub(crate) up       : Option<SceneGraphFrame>,
    pub(crate) draw     : Box<dyn FnMut(Point<i32>, &CanvasRenderingContext2d) -> Result<(), JsValue>>,
    pub(crate) width    : Box<dyn Fn() -> u32>,
    pub(crate) height   : Box<dyn Fn() -> u32>,
}

#[derive(Clone)]
pub struct SceneGraphLeaf(pub(crate) Shared<SceneGraphLeafInner>);

impl SceneGraphLeaf {
    pub fn new<D: DeferredDrawable>(defer: D, at: Option<Point<i32>>) -> Self {
        SceneGraphLeaf(Shared::new(SceneGraphLeafInner {
            pos     : at.unwrap_or_else(|| Point::<i32>::default()),
            up      : None,
            draw    : defer.deferred_draw(),
            width   : defer.deferred_width(),
            height  : defer.deferred_height(),
        }))
    }

    pub fn new_node<D: DeferredDrawable>(defer: D, at: Option<Point<i32>>) -> SceneGraphNode {
        SceneGraphNode::Leaf(Self::new(defer, at))
    }
}

macro_rules! mk_locally_positioned_by_pos {
    ($t:ident) => {
        impl LocallyPositioned for $t {
            #[inline(always)]
            fn x(&self) -> i32 {
                self.0.pos.x
            }

            #[inline(always)] 
            fn y(&self) -> i32 {
                self.0.pos.y
            }

            #[inline(always)]
            fn set_x(&mut self, x: i32) {
                self.0.pos.x = x;
            }

            #[inline(always)]
            fn set_y(&mut self, y: i32) {
                self.0.pos.y = y;
            }

            #[inline(always)]
            fn set_x_and_y(&mut self, x: i32, y: i32) {
                self.0.pos.set(x, y);
            }

            #[inline(always)]
            fn set_both_to(&mut self, v: i32) {
                self.0.pos.set_both(v);
            }
        }
    };
}

mk_locally_positioned_by_pos!{SceneGraphLeaf}

impl LocallyBounded for SceneGraphLeaf {
    fn local_bounds(&self) -> Rect<i32> {
        Rect::new_by_dim(
            self.y(),
            self.x(),
            (self.0.width)() as i32,
            (self.0.height)() as i32,
        )
    }
}

impl SceneGraph for SceneGraphLeaf {
    fn parent(&self) -> Option<&SceneGraphFrame> {
        match &self.0.up {
            &None           => None,
            &Some(ref frm)  => Some(frm),
        }
    }
}


#[derive(Clone)]
pub enum SceneGraphNode {
    Leaf(SceneGraphLeaf),
    Frame(SceneGraphFrame),
}

impl SceneGraphNode {
    pub(crate) fn set_parent(&mut self, parent: Option<SceneGraphFrame>) {
        use SceneGraphNode::*;
        match self {
            &mut Leaf(ref mut leaf)     => { leaf.0.up  = parent; },
            &mut Frame(ref mut frame)   => { frame.0.up = parent; },
        }
    }

    fn numeric_address(&self) -> usize {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref leaf)     => leaf.0.as_ptr()  as usize,
            &Frame(ref frame)   => frame.0.as_ptr() as usize,
        }
    }

    fn unwrap_leaf(self) -> SceneGraphLeaf {
        match self {
            SceneGraphNode::Leaf(leaf)  => leaf,
            _                           => panic!("SceneGraphNode::unwrap_leaf: not a leaf"),
        }
    }

    fn unwrap_frame(self) -> SceneGraphFrame {
        match self {
            SceneGraphNode::Frame(frame)    => frame,
            _                               => panic!("SceneGraphNode::unwrap_frame: not a frame"),
        }
    }

    fn pos(&self) -> &Point<i32> {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref leaf)     => &leaf.0.pos,
            &Frame(ref frame)   => &frame.0.pos,
        }
    }

    fn pos_mut(&mut self) -> &mut Point<i32> {
        use SceneGraphNode::*;
        match self {
            &mut Leaf(ref mut leaf)     => &mut leaf.0.pos,
            &mut Frame(ref mut frame)   => &mut frame.0.pos,
        }
    }

    fn parent_clone(&mut self) -> Option<SceneGraphFrame> {
        use SceneGraphNode::*;
        match self {
            &mut Leaf(ref mut leaf)     => leaf.0.up.clone(),
            &mut Frame(ref mut frm)     => frm.0.up.clone(),
        }
    }
}

impl LocallyPositioned for SceneGraphNode {
    #[inline(always)]
    fn x(&self) -> i32 {
        self.pos().x
    }

    #[inline(always)] 
    fn y(&self) -> i32 {
        self.pos().y
    }

    #[inline(always)]
    fn set_x(&mut self, x: i32) {
        self.pos_mut().x = x;
    }

    #[inline(always)]
    fn set_y(&mut self, y: i32) {
        self.pos_mut().y = y;
    }

    #[inline(always)]
    fn set_x_and_y(&mut self, x: i32, y: i32) {
        self.pos_mut().set(x, y);
    }

    #[inline(always)]
    fn set_both_to(&mut self, v: i32) {
        self.pos_mut().set_both(v);
    }
}

impl LocallyBounded for SceneGraphNode {
    fn local_bounds(&self) -> Rect<i32> {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref leaf)     => leaf.local_bounds(),
            &Frame(ref frame)   => frame.local_bounds(),
        }
    }
}

impl SceneGraph for SceneGraphNode {
    fn parent(&self) -> Option<&SceneGraphFrame> {
        use SceneGraphNode::*;
        match self {
            &Leaf(ref leaf)     => leaf.parent(),
            &Frame(ref frame)   => frame.parent(),
        }
    }
}


#[derive(Clone)]
pub(crate) struct SceneGraphFrameInner {
    pub(crate) up       : Option<SceneGraphFrame>,
    pub(crate) pos      : Point<i32>,
    pub(crate) clip     : Option<Rect<i32>>,
    pub(crate) children : Vec<SceneGraphNode>,
}

#[derive(Clone)]
pub struct SceneGraphFrame(pub(crate) Shared<SceneGraphFrameInner>);

impl SceneGraphFrame {
    const EXPECTED_MAX_CHILDREN: usize = 32;

    pub fn new(pos: Option<Point<i32>>, clip: Option<Rect<i32>>, children: Option<&[SceneGraphNode]>) -> Self {
        let children = match children {
            Some(children_slice)    => {
                let mut v = Vec::<SceneGraphNode>::with_capacity(children_slice.len() << 1);
                v.extend_from_slice(children_slice);
                v
            },
            _                       =>
                Vec::with_capacity(Self::EXPECTED_MAX_CHILDREN),
        };
        SceneGraphFrame(Shared::new(SceneGraphFrameInner {
            up      : None,
            pos     : pos.unwrap_or(Point::<i32>::default()),
            clip,
            children,
        }))
    }

    #[inline(always)]
    pub fn new_node(pos: Option<Point<i32>>, clip: Option<Rect<i32>>, children: Option<&[SceneGraphNode]>) -> SceneGraphNode {
        SceneGraphNode::Frame(Self::new(pos, clip, children))
    }

    fn find_index(&self, child: &SceneGraphNode) -> Result<usize, ()> {
        let lhs = child.numeric_address();
        //self.0.children.binary_search_by(|child| {
        //    let rhs = child.numeric_address();
        //    lhs.cmp(&rhs)
        //})
        for i in 0 .. self.0.children.len() {
            if self.0.children[i].numeric_address() == lhs {
                return Ok(i)
            }
        }
        Err(())
    }

    pub fn append(&mut self, mut child: SceneGraphNode) -> Result<(), SceneGraphNode> {
        if let Some(mut par) = child.parent_clone() {
            par.remove(&mut child);
        }
        match self.find_index(&child) {
            Err(_)    => {
                child.set_parent(Some(self.clone()));
                //self.0.children.insert(idx, child);
                self.0.children.push(child);
                Ok(())
            },
            _           =>
                Err(child),
        }
    }

    pub fn insert(&mut self, idx: usize, mut child: SceneGraphNode) -> Result<(), SceneGraphNode> {
        match self.find_index(&child) {
            Ok(_)   => Err(child),
            Err(_)  => {
                child.set_parent(Some(self.clone()));
                self.0.children.insert(idx, child.clone());
                Ok(())
            }
        }
    }

    #[inline]
    pub fn append_leaf(&mut self, leaf: SceneGraphLeaf) -> Result<(), SceneGraphLeaf> {
        self.append(SceneGraphNode::Leaf(leaf)).map_err(|wrap| wrap.unwrap_leaf())
    }

    #[inline]
    pub fn append_frame(&mut self, frame: SceneGraphFrame) -> Result<(), SceneGraphFrame> {
        self.append(SceneGraphNode::Frame(frame)).map_err(|wrap| wrap.unwrap_frame())
    }

    pub fn append_new_drawable<D: DeferredDrawable>(
        &mut self,
        defer   : D,
        at      : Option<Point<i32>>
    ) -> SceneGraphLeaf
    {
        let mut leaf    = SceneGraphLeaf::new(defer, at);
        let leaf_wrap   = SceneGraphNode::Leaf(leaf.clone());
        leaf.0.up       = Some(self.clone());
        //let idx         = self.find_index(&leaf_wrap).unwrap_err();
        //self.0.children.insert(idx, leaf_wrap);
        self.0.children.push(leaf_wrap);
        leaf
    }

    pub fn append_new_frame(
        &mut self,
        pos : Option<Point<i32>>,
        clip: Option<Rect<i32>>
    ) -> SceneGraphFrame
    {
        let child           = Self::new(pos, clip, None);
        let child_wrapped   = SceneGraphNode::Frame(child.clone());
        //let idx             = self.find_index(&child_wrapped).unwrap_err();
        //self.0.children.insert(idx, child_wrapped);
        self.0.children.push(child_wrapped);
        child
    }

    pub fn children(&self) -> &[SceneGraphNode] {
        &self.0.children[..]
    }

    pub fn children_mut(&mut self) -> &mut Vec<SceneGraphNode> {
        &mut self.0.children
    }

    pub fn remove(&mut self, child: &mut SceneGraphNode) -> bool {
        match self.find_index(child) {
            Ok(idx) => {
                self.0.children.remove(idx);
                child.set_parent(None);
                true
            },
            _       =>
                false,
        }
    }

    pub fn remove_at(&mut self, idx: usize) -> Result<SceneGraphNode, ()> {
        if idx >= self.0.children.len() {
            Err(())
        }
        else {
            Ok(self.0.children.remove(idx))
        }
    }
}

mk_locally_positioned_by_pos!{SceneGraphFrame}

impl LocallyBounded for SceneGraphFrame {
    fn local_bounds(&self) -> Rect<i32> {
        self.0.children
            .iter()
            .map(|child| {
                child.local_bounds() + &self.0.pos
            })
            .fold(
                Rect::new(i32::MAX, i32::MAX, i32::MIN, i32::MIN),
                |acc, curr|
                    Rect::new(
                        min(acc.top,    curr.top),
                        min(acc.left,   curr.left),
                        max(acc.bottom, curr.bottom),
                        max(acc.right,  curr.right)
                    )
            )
    }
}

impl SceneGraph for SceneGraphFrame {
    fn parent(&self) -> Option<&SceneGraphFrame> {
        match &self.0.up {
            &None           => None,
            &Some(ref par)  => Some(par),
        }
    }
}
