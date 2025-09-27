//! Module  :   BaseHierarchical
//! Purpose :   An object that provides an optional parent and with it
//!             the possibility of cumulative ("*world*") transforms.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Fri 09/10/2021 12:18 AM
//! Modified:   Mon 09/13/2021 10:20 AM


use crate::math::geom::transform::Transform;
use super::frame::SceneGraphFrame;


/// The structure holding a scene-graph hierarchy -- a
/// `Transformable` itself -- also has an optional parent
/// and a *world* transform and its inverse. Much like
/// a `Transformable`, they are not constantly
/// synchronized, but the information is updated as
/// needed by the user or the rendering engine.
#[derive(Clone, Debug)]
pub struct BaseHierarchical<N> {
    /// Optional parent.
    pub(crate)  par         : Option<SceneGraphFrame<N>>,
    /// World transform.
    pub(crate)  world       : Transform<N>,
    /// Inverse world transform.
    pub(crate)  world_inv   : Transform<N>,
}


impl<N> BaseHierarchical<N>
    where   N   :   Default + From<i16>
                +   Copy
{
    pub fn new() -> Self {
        BaseHierarchical {
            par         : None,
            world       : Transform::<N>::new_identity(),
            world_inv   : Transform::<N>::new_identity(),
        }
    }
}


/// A trait for building hierarchical
/// functionality through composition.
pub trait HasBaseHierarchical<N> {
    fn base_hierarchical(&self) -> &BaseHierarchical<N>;
    fn base_hierarchical_mut(&self) -> &mut BaseHierarchical<N>;
}
