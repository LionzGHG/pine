
use std::{cell::{Ref, RefCell, RefMut}, rc::Rc};

use crate::{Attribute, Commands, Component, ComponentPointer, math::Point, upload};

/// ## Description
/// The [KeyCode] struct provides constants that are associated to the representing [i32]-value for the
/// respective key.
/// ## Example
/// ```
/// fn key_callback(commands: &mut Commands, keycode: i32) {
///     if keycode == KeyCode::UP {
///         let mut player = find!(commands, Actor, "Player");
///         player.transform.position.y += 10;
///     }
/// }
/// ```
pub struct KeyCode;

impl KeyCode {
    pub const BACK: i32 = 8;
    pub const ENTER: i32 = 13;
    pub const SHIFT: i32 = 16;
    pub const ESCAPE: i32 = 27;
    pub const SPACE: i32 = 32;

    pub const LEFT: i32 = 1073741904;
    pub const UP: i32 = 1073741906;
    pub const RIGHT: i32 = 1073741903;
    pub const DOWN: i32 = 1073741905;

    pub const LMB: i32 = 1;
    pub const RMB: i32 = 3;
    // TODO
}

pub trait ToPoint {
    fn to_point(&self) -> Point;
}

impl ToPoint for (i32, i32) {
    fn to_point(&self) -> Point {
        Point(self.0, self.1)
    }
}

/// ## Description
/// Implement the `Globalize` trait for your datatypes to access the [make_global](Globalize::make_global) method to instantly
/// turn any object that inherits this trait into a [global variable](Commands::add_global_var).
pub trait Globalize {
    /// ## Description
    /// This function turns any local variable into a global variable by adding it to the list of global variables of the
    /// [Commands] control structure.
    /// You can access global variables via. the [load](crate::load) macro or by using the [get_global_var](Commands::get_global_var) method of Commands.
    /// ## Example
    /// ```
    /// fn start(commands: &mut Commands) {
    ///     let counter = 0u32;
    ///     counter.make_global(commands);
    /// }
    /// ```
    fn make_global(&self, commands: &mut Commands);
}

impl Globalize for i32 {
    fn make_global(&self, commands: &mut Commands) {
        upload!(commands, self);
    }
}

impl Globalize for f64 {
    fn make_global(&self, commands: &mut Commands) {
        upload!(commands, self);
    }
}

impl Globalize for String {
    fn make_global(&self, commands: &mut Commands) {
        upload!(commands, self);
    }
}

impl Globalize for u32 {
    fn make_global(&self, commands: &mut Commands) {
        upload!(commands, self);
    }
}

pub trait AttributeSafecast {
    fn safecast_ref_mut<T: 'static + Attribute>(&self) -> Option<RefMut<'_, T>>;
    fn safecast_ref<T: 'static + Attribute>(&self) -> Option<Ref<'_, T>>;
}

impl AttributeSafecast for Rc<RefCell<dyn Attribute>> {
    fn safecast_ref_mut<T: 'static + Attribute>(&self) -> Option<RefMut<'_, T>> {
        let borrow = self.borrow_mut();

        if borrow.as_any().is::<T>() {
            Some(RefMut::map(borrow, |c| {
                c.as_any_mut().downcast_mut::<T>().unwrap()
            }))
        } else {
            None
        }
    }

    fn safecast_ref<T: 'static + Attribute>(&self) -> Option<Ref<'_, T>> {
        let borrow = self.borrow();

        if borrow.as_any().is::<T>() {
            Some(Ref::map(borrow, |c| {
                c.as_any().downcast_ref::<T>().unwrap()
            }))
        } else {
            None
        }
    }
}

pub trait AnySafecast {
    fn safecast_ref<T: 'static + std::any::Any>(&self) -> Option<RefMut<'_, T>>;
}

impl AnySafecast for Rc<RefCell<dyn std::any::Any + 'static>> {
    fn safecast_ref<T: 'static + std::any::Any>(&self) -> Option<RefMut<'_, T>> {
        let borrow = self.borrow_mut();

        if borrow.is::<T>() {
            Some(RefMut::map(borrow, |c| {
                c.downcast_mut::<T>().unwrap()
            }))
        } else {
            None
        }
    }
}

// ##### COMPONENT UTILS ##### 

pub trait ComponentCast<T> {
    fn cast(&self) -> Option<&T>;
}

pub trait ComponentSafecast {
    fn safecast<T: 'static + Component>(&self) -> Option<&T>;
}

pub trait ComponentReferenceSafecast {
    fn safecast_ref<T: 'static + Component>(&self) -> Option<RefMut<'_, T>>;
}

impl<T: 'static + Component> ComponentCast<T> for Box<dyn Component> {
    fn cast(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl ComponentReferenceSafecast for &ComponentPointer {
    fn safecast_ref<T: 'static + Component>(&self) -> Option<RefMut<'_, T>> {
        let borrow = self.borrow_mut();

        if borrow.as_any().is::<T>() {
            Some(RefMut::map(borrow, |c| {
                c.as_any_mut().downcast_mut::<T>().unwrap()
            }))
        } else {
            None
        }
    }
}

impl ComponentReferenceSafecast for Rc<RefCell<dyn Component + 'static>> {
    fn safecast_ref<T: 'static + Component>(&self) -> Option<RefMut<'_, T>> {
        let borrow = self.borrow_mut();

        if borrow.as_any().is::<T>() {
            Some(RefMut::map(borrow, |c| {
                c.as_any_mut().downcast_mut::<T>().unwrap()
            }))
        } else {
            None
        }
    }
}

impl ComponentSafecast for Box<dyn Component> {
    fn safecast<T: 'static + Component>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

#[inline(always)]
pub fn cstr_rb() -> *const i8 {
    b"rb\0".as_ptr() as *const i8
}

pub trait Floatify {
    fn floatify(&self) -> f32;
}

impl Floatify for f32 {
    fn floatify(&self) -> f32 {
        *self
    }
}

impl Floatify for i32 {
    fn floatify(&self) -> f32 {
        *self as f32
    }
}