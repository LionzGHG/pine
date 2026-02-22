
use std::{cell::{Ref, RefCell, RefMut}, rc::Rc, sync::{Mutex, OnceLock}, time::Instant};

use crate::{Attribute, Commands, Component, ComponentPointer, math::Point, upload, Engine};

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
    /// fn start() -> Result<(), RuntimeException> {
    ///     let counter = 0u32;
    ///     counter.make_global("counter")?;
    /// }
    /// ```
    fn make_global(&self, id: &'static str) -> Result<(), RuntimeException>;
}

impl Globalize for i32 {
    fn make_global(&self, id: &'static str) -> Result<(), RuntimeException> {
        upload!(id, self)
    }
}

impl Globalize for f64 {
    fn make_global(&self, id: &'static str ) -> Result<(), RuntimeException> {
        upload!(id, self)
    }
}

impl Globalize for String {
    fn make_global(&self, id: &'static str) -> Result<(), RuntimeException> {
        upload!(id, self)
    }
}

impl Globalize for u32 {
    fn make_global(&self, id: &'static str) -> Result<(), RuntimeException> {
        upload!(id, self)
    }
}

pub trait AttributeSafecast {
    fn safecast_ref_mut<T: 'static + Attribute>(&self) -> Option<RefMut<'_, T>>;
    fn safecast_ref<T: 'static + Attribute>(&self) -> Option<Ref<'_, T>>;
}

pub trait Downcastable {
    fn can_be_downcast_to<T: 'static + Attribute>(&self) -> bool;
}

impl Downcastable for Rc<RefCell<dyn Attribute>> {
    fn can_be_downcast_to<T: 'static + Attribute>(&self) -> bool {
        let borrow = self.borrow();

        if borrow.as_any().is::<T>() {
            drop(borrow);
            true
        } else {
            drop(borrow);
            false
        }        
    }
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

pub trait ComponentSafecastRc {
    fn safecast_rc<T: Component + 'static>(&self) -> Option<Rc<RefCell<T>>>;
}

pub trait AttributeSafecastRc {
    fn safecast_rc<T: Attribute + 'static>(&self) -> Option<Rc<RefCell<T>>>;
}

impl AttributeSafecastRc for Rc<RefCell<dyn Attribute>> {
    fn safecast_rc<T: Attribute + 'static>(&self) -> Option<Rc<RefCell<T>>> {
        if self.borrow().as_any().is::<T>() {
            let raw = Rc::into_raw(self.clone()) as *const RefCell<T>;
            Some(unsafe { Rc::from_raw(raw) })
        } else {
            None
        }
    }
}

impl ComponentSafecastRc for Rc<RefCell<dyn Component>> {
    fn safecast_rc<T: Component + 'static>(&self) -> Option<Rc<RefCell<T>>> {
        if self.borrow().as_any().is::<T>() {
            let raw = Rc::into_raw(self.clone()) as *const RefCell<T>;
            Some(unsafe { Rc::from_raw(raw) })
        } else {
            None
        }
    }
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

pub struct RuntimeException(pub String);

impl RuntimeException {
    pub fn emit(&self) {
        println!("[Pine] RUNTIME EXCEPTION: {}", self.0)
    }

    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

pub trait IntoResult<T> {
    fn into_result(&self, msg: &'static str) -> Result<(), RuntimeException>;
    fn into_result_string(&self, msg: String) -> Result<(), RuntimeException>;
    fn into_result_value(&self, msg: String) -> Result<T, RuntimeException>;
}

impl<T: Clone> IntoResult<T> for Option<T> {
    fn into_result(&self, msg: &'static str) -> Result<(), RuntimeException> {
        if self.is_some() {
            Ok(())
        } else {
            Err(RuntimeException(msg.to_string()))
        }
    }

    fn into_result_value(&self, msg: String) -> Result<T, RuntimeException> {
        if let Some(value) = self {
            Ok(value.clone())
        } else {
            Err(RuntimeException(msg))
        }
    }

    fn into_result_string(&self, msg: String) -> Result<(), RuntimeException> {
        if self.is_some() {
            Ok(())
        } else {
            Err(RuntimeException(msg))
        }
    }
}

pub struct Time;

struct TimeState {
    last_frame: Instant,
    delta: f32,
}

static TIME_STATE: OnceLock<Mutex<TimeState>> = OnceLock::new();

impl Time {
    pub fn init() {
        TIME_STATE.get_or_init(|| {
            Mutex::new(TimeState {
                last_frame: Instant::now(),
                delta: 0.
            })
        });
    }

    pub fn update() {
        let state = TIME_STATE
            .get()
            .expect("Time::init() must be called before Time::update()");

        let mut state = state.lock().unwrap();

        let now = Instant::now();
        let dt = now.duration_since(state.last_frame);

        state.delta = dt.as_secs_f32();
        state.last_frame = now;
    }

    pub fn delta() -> f32 {
        let state = TIME_STATE
            .get()
            .expect("Time::init() must be called before Time::delta()");

        state.lock().unwrap().delta
    }

    pub fn delta_f64() -> f64 {
        Self::delta() as f64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub const BLACK: Color = Color(0, 0, 0, 255);
    pub const WHITE: Color = Color(255, 255, 255, 255);
    pub const RED: Color = Color(255, 0, 0, 255);
    pub const GREEN: Color = Color(0, 255, 0, 255);
    pub const BLUE: Color = Color(0, 0, 255, 255);
    pub const YELLOW: Color = Color(255, 255, 0, 255);
    pub const CYAN: Color = Color(0, 255, 255, 255);
    pub const MAGENTA: Color = Color(255, 0, 255, 255);
    pub const GRAY: Color = Color(128, 128, 128, 255);
    pub const ORANGE: Color = Color(255, 165, 0, 255);

    #[inline(always)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r,g,b,a)
    }
}

impl From<sdl2::pixels::Color> for Color {
    fn from(value: sdl2::pixels::Color) -> Self {
        Color(value.r, value.g, value.b, value.a)
    }
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGBA(self.0, self.1, self.2, self.3)
    }
}