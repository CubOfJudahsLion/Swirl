//! Module  :   Shared
//! Purpose :   A reference-counted shareable resource.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu Mar 11 11:55:43 AM CST 2021
//! Modified:   Wed Sep  1 06:07:56 PM CST 2021


use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;


#[derive(Debug)]
pub struct Shared<T>(Rc<RefCell<T>>);

impl<T> Shared<T> {
    #[inline]
    pub fn new(t: T) -> Self {
        Shared(Rc::new(RefCell::new(t)))
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        &*self.0.deref().borrow() as *const T
    }

    pub fn borrow_mut(&self) -> &mut T {
        unsafe {
            &mut *self.0.deref().as_ptr()
        }
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0.deref().as_ptr()
        }
    }
}

impl<T> DerefMut for Shared<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.0.deref().as_ptr()
        }
    }
}

impl<T> Clone for Shared<T> {
    #[inline]
    fn clone(&self) -> Self {
        Shared(self.0.clone())
    }
}

//  Note: this was the old code.
//  The implementation was fast but naïve: it overrode Rust safeguards and
//  internal type knowledge.

// /// A smart (reference-counted) pointer that can be shared among several owners.
// /// This structure is specifically designed for single-threaded applications,
// /// which suits SPAs like ours.
// /// This was created to both reduce the overhead of having a double-wrapper
// /// (i.e., `Rc<RefCell<T>>`) with extra checks we don't need: `Rc` implements
// /// the one-mutable, many-readable semantic in a way that a persistent `Rc`'s
// /// `deref_mut` prevents accesses at all other shared instances. While `RefCell`
// /// remedies that with a `borrow_mut` out of a `&Rc`, that's already going
// /// through too many checks. Our type implements the **C++**-style 
// // /// owners. Made to simplify the creation of a `Rc<RefCell<T>>`.
// struct SharedKeeper<T> {
//     ref_count   : usize,
//     datum       : T,
// }
// 
// impl<T> SharedKeeper<T> {
//     fn new(datum: T) -> *mut SharedKeeper<T> {
//         Box::into_raw(
//             Box::new(SharedKeeper {
//                 ref_count   : 1,
//                 datum
//             })
//         )
//     }
// 
//     fn release(keeper: *mut SharedKeeper<T>) {
//         let _ = unsafe { Box::from_raw(keeper) };
//     }
// }
// 
// pub struct Shared<T> {
//     keeper  : *mut SharedKeeper<T>,
// }
// 
// impl<T> Shared<T> {
//     /// Create a new `Shared<T>`.
//     #[inline]
//     pub fn new(datum: T) -> Self {
//         Shared{keeper: SharedKeeper::new(datum)}
//     }
// 
//     /// Return a pointer to the contained `T` instance.
//     pub fn as_ptr(&self) -> *const T {
//         unsafe { &(*self.keeper).datum }
//     }
// }
// 
// impl<T> Clone for Shared<T> {
//     fn clone(&self) -> Self {
//         // Cloning merely increases the reference count,
//         // and returns another object with the same pointer.
//         unsafe { (*self.keeper).ref_count += 1; }
//         Shared {keeper: self.keeper}
//     }
// }
// 
// impl<T> Drop for Shared<T> {
//     fn drop(&mut self) {
//         if unsafe { (*self.keeper).ref_count == 1 } {
//             //  Just delete if last one.
//             SharedKeeper::release(self.keeper);
//         }
//         else {
//             //  Otherwise, just reduce the reference count.
//             unsafe { (*self.keeper).ref_count -= 1; }
//         }
//     }
// }
// 
// impl<T> Deref for Shared<T> {
//     type Target = T;
// 
//     #[inline(always)]
//     fn deref(&self) -> &Self::Target {
//         unsafe { &(*self.keeper).datum }
//     }
// }
// 
// impl<T> DerefMut for Shared<T> {
//     #[inline(always)]
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         unsafe { &mut (*self.keeper).datum }
//     }
// }
// 
