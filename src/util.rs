
use std::{cell::{Ref, RefCell, RefMut}, rc::Rc, sync::{Mutex, OnceLock}, time::Instant};

use crate::{Attribute, Commands, Component, ComponentPointer, Engine, assets::{Asset, AssetExt}, math::Point, upload};

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
    
    pub fn key_name_of(keycode: i32) -> &'static str {
        match keycode {
            8 => "BACK",
            13 => "ENTER",
            16 => "SHIFT",
            27 => "ESCAPE",
            32 => "SPACE",
            104 => "H",
            115 => "S",
            100 => "D",
            1073741904 => "LEFT",
            1073741906 => "UP",
            1073741903 => "RIGHT",
            1073741905 => "DOWN",
            1 => "LMB",
            3 => "RMB",
            _ => "UNKOWN" 
        }
    }

    pub const KEY_H: i32 = 104;
    pub const KEY_S: i32 = 115;
    pub const KEY_D: i32 = 100;

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

/// ## Description
/// Utility trait to convert tuple-based coordinate data into a [Point](crate::math::Point).
/// 
/// In the context of the engine, this is primarily used for ergonomic
/// conversion of raw integer coordinate pairs (e.g. mouse input, grid
/// positions, SDL values) into the engine’s internal **math::Point** type.
/// 
/// This avoids manual construction like `Point(x, y)` and allows writing
/// `(x, y).to_point()` instead.
/// 
/// - **Item-Type**: Utility Conversion Trait
/// 
/// ## Example
/// ```
/// let mouse_pos = (mouse_x, mouse_y).to_point();
/// actor.transform.position = mouse_pos;
/// ```
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
        Engine::add_global_var(id, self)
    }
}

impl Globalize for bool {
    fn make_global(&self, id: &'static str) -> Result<(), RuntimeException> {
        Engine::add_global_var(id, self)
    }
}

impl Globalize for f64 {
    fn make_global(&self, id: &'static str ) -> Result<(), RuntimeException> {
        Engine::add_global_var(id, self)
    }
}

impl Globalize for String {
    fn make_global(&self, id: &'static str) -> Result<(), RuntimeException> {
        Engine::add_global_var(id, self)
    }
}

impl Globalize for u32 {
    fn make_global(&self, id: &'static str) -> Result<(), RuntimeException> {
        Engine::add_global_var(id, self)
    }
}

/// ## Description
/// Provides safe runtime downcasting utilities for [Attribute] trait objects
/// wrapped inside `Rc<RefCell<dyn Attribute>>`.
/// 
/// This trait is essential for the engine’s dynamic attribute system,
/// allowing attributes to be queried and accessed without unsafe code.
/// 
/// It enables:
/// - Mutable downcasting via `safecast_ref_mut`
/// - Immutable downcasting via `safecast_ref`
///
/// Used internally by systems and components when interacting with
/// heterogeneous attribute collections.
/// 
/// - **Item-Type**: Attribute Runtime Cast Utility
/// 
/// ## Example
/// ```
/// if let Some(mut collider) = attr.safecast_ref_mut::<Collision2D>() {
///     collider.size.x += 10.0;
/// }
/// ```
pub trait AttributeSafecast {
    fn safecast_ref_mut<T: 'static + Attribute>(&self) -> Option<RefMut<'_, T>>;
    fn safecast_ref<T: 'static + Attribute>(&self) -> Option<Ref<'_, T>>;
}

/// ## Description
/// Runtime type inspection helper for [Attribute] trait objects.
/// 
/// Allows checking whether a dynamic attribute can be downcast to a
/// specific concrete attribute type without performing the cast itself.
/// 
/// This is primarily used internally by engine casting utilities and
/// avoids unnecessary borrow mapping if the type does not match.
/// 
/// - **Item-Type**: Attribute Type Introspection Utility
/// 
/// ## Example
/// ```
/// if attribute.can_be_downcast_to::<Collision2D>() {
///     println!("Actor has a collider.");
/// }
/// ```
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

/// ## Description
/// Small numeric utility trait used to normalize numeric types into [f32].
/// 
/// In the engine, many rendering and physics APIs operate on `f32`.
/// This trait allows both `f32` and `i32` values to be passed into
/// APIs that expect floating-point values without explicit casting.
/// 
/// - **Item-Type**: Numeric Conversion Utility
/// 
/// ## Example
/// ```
/// let speed: i32 = 10;
/// let velocity = speed.floatify(); // becomes 10.0f32
/// ```
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

/// ## Description
/// Represents a runtime-level engine exception.
/// 
/// This type is used for recoverable runtime errors that occur during:
/// - Global variable access
/// - Asset loading
/// - Command execution
/// - Attribute interaction
/// 
/// Unlike Rust panics, this error is meant to be handled or emitted
/// gracefully inside the engine runtime.
/// 
/// - **Item-Type**: Engine Runtime Error
/// 
/// ## Example
/// ```
/// return Err(RuntimeException::new("Player not found"));
/// ```
#[derive(Debug)]
pub struct RuntimeException(pub String);

impl RuntimeException {
    pub fn emit(&self) {
        println!("[Pine] RUNTIME EXCEPTION: {}", self.0)
    }

    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

/// ## Description
/// Utility trait for converting [Option<T>] into engine-compatible
/// [Result] types with [RuntimeException].
/// 
/// This simplifies error propagation in engine code and avoids repetitive
/// `match` statements.
/// 
/// It allows:
/// - Converting existence checks into `Result<(), RuntimeException>`
/// - Extracting values safely
/// 
/// - **Item-Type**: Error Handling Utility
/// 
/// ## Example
/// Converting an existing [Result]-type into an Pine-compatible Result:
/// ```
/// some_action_that_might_yield_result.into_result("Error message");
/// ```
/// Turning an [Option<T>] into a Pine-compatible [RuntimeException]-Result:
/// ```
/// let y: Option<i32> = None;
/// let x: Result<i32, RuntimeException> = y.into_result_value("Error: failed");
/// let z: i32 = x?; // Throws RuntimeException - 'Error: failed'
/// ```
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

/// ## Description
/// Global time management utility of the engine.
/// 
/// Responsible for tracking frame-to-frame delta time and providing
/// time-step information to physics, animation, and update systems.
/// 
/// Must be initialized once during engine startup using `Time::init()`.
/// The engine loop must call `Time::update()` once per frame.
/// 
/// - **Item-Type**: Global Engine Timing System
/// 
/// ## Example
/// ```
/// Time::init();
///
/// loop {
///     Time::update();
///     let dt = Time::delta();
///     player.velocity += 100.0 * dt;
/// }
/// ```
///
/// ## Technical Info
/// Internally uses `OnceLock<Mutex<TimeState>>` to ensure:
/// - Single initialization
/// - Thread-safe delta updates
pub struct Time;

/// ## Description
/// Global time management utility of the engine.
/// 
/// Responsible for tracking frame-to-frame delta time and providing
/// time-step information to physics, animation, and update systems.
/// 
/// Must be initialized once during engine startup using `Time::init()`.
/// The engine loop must call `Time::update()` once per frame.
/// 
/// - **Item-Type**: Global Engine Timing System
/// 
/// ## Example
/// ```
/// Time::init();
///
/// loop {
///     Time::update();
///     let dt = Time::delta();
///     player.velocity += 100.0 * dt;
/// }
/// ```
///
/// ## Technical Info
/// Internally uses `OnceLock<Mutex<TimeState>>` to ensure:
/// - Single initialization
/// - Thread-safe delta updates
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
                delta: 0.,
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

/// ## Description
/// Represents an RGBA color used throughout the rendering system.
/// 
/// The engine uses this type instead of [sdl2::pixels::Color] to
/// decouple core engine logic from SDL-specific types.
/// 
/// Provides predefined constants for common colors and supports
/// conversion to/from SDL color types.
/// 
/// - **Item-Type**: Rendering Primitive
/// 
/// ## Example
/// ```
/// fn start() -> Result<(), RuntimeException> {
///     let actor = make!(Actor::new("My Actor", "")); // actor with empty texture -> Material-based
///     
///     Engine::capture(actor.clone(), |actor| {
///         actor.set_size(100, 100);   
///         actor.set_color(Color::RED); // set material color to red
///     });
/// 
///     Engine::spawn(actor)?;
///     Ok(())
/// }
/// ```
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
}/// # Description
/// Extends to `Result<(), RuntimeException>`.
pub type PineResult = Result<(), RuntimeException>;