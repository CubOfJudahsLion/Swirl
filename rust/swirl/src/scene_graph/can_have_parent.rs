//! Module  :   Can-Have-Parent
//! Purpose :   A building-block scene graph trait.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Tue Mar 16 08:20:07 PM CST 2021
//! Modified:   Thu Jul 22 01:44:01 PM CST 2021


use super::SceneGraphFrame;


/// A scene graph object that can have a parent (which can only be
/// a `SceneGraphFrame`, see below.)
pub trait CanHaveParent<N> {
    fn parent(&self) -> &Option<SceneGraphFrame<N>>;
    fn parent_mut(&self) -> &mut Option<SceneGraphFrame<N>>;
}
