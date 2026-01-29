
use std::{cell::{Ref, RefCell, RefMut}, rc::Rc};

use crate::{Component, ComponentPointer};

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