//! Module  :   TransformableState
//! Purpose :   Describes the inner coherency state of
//!             a `Transformable` object and sub-traits.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Fri Sep 10 11:02:25 AM CST 2021


/// This is a type to inform of the state of transform
/// updates. The value is `Dirty` when any of the values
/// (pivot, translation, rotation, etc.) is updated, and
/// the transform as a whole needs updating. When the
/// local transform is updated according to said changes,
/// its value should change to `LocallyUpdated`. Then
/// the transforms are applied hierarchically, yielding
/// *world* transforms, for each object, and setting
/// the state should be `GloballyUpdated`. Finally, if
/// the user requires the related calculations, the
/// inverse of the world transform is calculated and
/// cached, and the state changes to
/// `GloballyUpdatedWithInverse`.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TransformableState {
    Dirty,
    LocallyUpdated,
    GloballyUpdated,
    GloballyUpdatedWithInverse,
}
