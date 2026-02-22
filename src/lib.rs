//! # Welcome to Pine!
//! ***Pine*** is a small, easy to use 2d game engine for rust. To start your application, orient yourself according
//! to this setup guide!
//! ## Setup-Guide
//! To start with your application, follow this general application layout:
//! ```
//! fn main() {
//!     let window = Window::new_no_commands("My Window", __start__, __update__);
//!     window.run();
//! }
//! 
//! fn __start__() -> Result<(), RuntimeException> {
//!     println!("Program has started!");
//! }
//! 
//! fn __update__() -> Result<(), RuntimeException> {
//!     println!("Program is running!");
//! }
//! ```
//! ## Handling Events
//! **event handling** in Pine is achieved through callbacks. Use `window.*_no_commands()` to add callback functions to your Window.
//! 
//! Some useful callbacks are:
//! - `on_key_down` and `on_key_down_no_commands`
//! - `on_mouse_button_down` and `on_mouse_button_down_no_commands`
//! - `on_mouse_motion` and `on_mouse_motion_no_commands`
//! ### Usage Exaple (without commands, **recommended!**)
//! ```
//! fn main() {
//!     let mut window = Window::new_no_commands("my window", start, update);
//!     window.on_key_down_no_commands(key_down);
//!     window.run();
//! }
//! 
//! fn key_down(keycode: i32) -> Result<(), RuntimeException> {
//!     if keycode == Keycode::Space {
//!         println!("Space Key pressed!");    
//!     }
//! 
//!     Ok(())
//! }
//! ```
//! ### Usage Example (with commands)
//! ```
//! fn main() {
//!     let mut window = Window::new("my window", start, update);
//!     window.on_key_down(key_down);
//!     window.run();
//! }
//! 
//! fn key_down(commands: &mut Commands, keycode: i32) -> Result<(), RuntimeException> {
//!     if keycode == Keycode::Space {
//!         println!("Space key pressed!")
//!     }
//! 
//!     Ok(())
//! }
//! ```
//! ## Info
//! For further orientation, get yourself acquainted with the [Window](window::Window), [Component] and [Engine] structs, as well as
//! the [Actor](basic_components::Actor) [Component]. 
//! ## Minimal Example
//! Below is a minimal example of a game, where you can control a player.
//! 
//! **Notice**: You'll have to place an `.png` or `.jpg` file at `./assets/textures/player`!
//! ```
//! use pine::prelude::*;
//! 
//! fn main() {
//!     let mut window = Window::new_no_commands("My Window", start, update);
//!     window.on_key_down_no_commands(on_key_down);
//!     window.run();
//! }
//! 
//! pub fn on_key_down(keycode: i32) -> Result<(), RuntimeException> {
//!     let mut player = Engine::find::<Actor>("Player");
//! 
//!     if keycode == KeyCode::UP {    
//!         player.transform.y -= 10;
//!     }
//!     if keycode == KeyCode::DOWN {
//!         player.transform.y += 10;
//!     }
//!     if keycode == KeyCode::RIGHT {
//!         player.transform.x += 10;
//!     }
//!     if keycode == KeyCode::LEFT {
//!         player.transform.x -= 10;
//!     }
//! }
//! 
//! pub fn start() -> Result<(), RuntimeException> {
//!     let player = make!(Actor::new("Player", "player"));
//!     Engine::spawn(player)?;
//! } 
//! 
//! pub fn update() -> Result<(), RuntimeException> {}
//! ``` 

pub type ComponentPointer = Rc<RefCell<dyn Component>>;

pub mod basic_components;
pub mod window;
pub mod prelude;
pub mod util;
pub mod assets;
pub mod basic_attributes;
pub mod math;
pub mod physics;

use std::cell::{RefCell, RefMut};
use std::ffi::CString;
use std::fmt::Display;
use std::rc::Rc;

thread_local! {
    static ACTIVE_COMMANDS: RefCell<Option<*mut Commands>> = const { RefCell::new(None) };
}

/// ## Discription
/// The [Engine] struct provides the necessary tools for controlling your application. 
/// 
/// In previous versions of ***Pine***, the [Commands] struct was used to control your application. Now, the Engine **wrapper**
/// Provides an **easier to use** alternative.
/// 
/// When creating your application, use `Window::new_no_commands` instead of `Window::new` to create a new application.
/// Your [start](crate::window::Window::start_no_commands) and [update](crate::window::Window::update_no_commands) will now
/// ***not*** excpect `&mut Commands` as an argument anymore.
/// 
/// To use the functionalities of [Commands], just use [Engine] instead.
/// 
/// ## Technical Info
/// [Engine] creates small scopes within your actual code where [Commands] is borrowed locally:
/// ```
/// Engine::with_commands(|cmds| {
///     // use commands here...
/// }).unwrap(); // commands dropped here again...
/// ``` 
/// For **static borrows**, use `Engine::with_commands_static` instead.
/// ## Utility Methods
/// However, [Engine] also provides the same functionalities as [Commands], for example, the [spawn](Engine::spawn) method:
/// ```
/// Engine::spawn(my_component)?;
/// ``` 
/// ## Example
/// ```
/// fn main() {
///     let window = Window::new_no_commands("My Window", start, update);
///     window.on_mouse_button_down_no_commands(on_mouse);
///     window.run(); // starts the program
/// }
/// 
/// fn on_mouse(_keycode: i32, _p: Point) {
///     let mut score = Engine::get_global_var::<u32>("score")?;
///     *score += 1;
/// }
/// 
/// fn start() -> Result<(), RuntimeException> {
///     let actor = make!(Actor::new("Player"));
///     Engine::spawn(actor)?;
/// 
///     let score = 0u32;
///     Engine::add_global_var("score", &score)?;
/// }
/// 
/// fn update() -> Result<(), RuntimeException> {
///     // ...    
/// }
/// ```
pub struct Engine;

impl Engine {
    pub fn get_world_size() -> (f32, f32) {
        Engine::with_commands(|cmds| cmds.world_size).unwrap()
    }

    pub fn get_height() -> f32 {
        Engine::get_world_size().1
    }

    pub fn get_width() -> f32 {
        Engine::get_world_size().0
    }

    pub fn get_world_center() -> crate::math::Vec2 {
        Engine::with_commands(|cmds| {
            let hw = cmds.world_size.0 / 2.;
            let hh = cmds.world_size.1 / 2.;
            crate::math::Vec2::new(hw, hh)
        }).unwrap()
    }

    pub fn capture<F>(actor: Rc<RefCell<crate::prelude::Actor>>, f: F)
    where
        F: FnOnce(&mut crate::prelude::Actor),
    {
        let mut borrowed = actor.borrow_mut();
        f(&mut borrowed);
    }

    /// ## Description
    /// The [find](Engine::find) method of the [Engine] struct allows you to search for currently active
    /// [Components](Component) and return them if found. You search for Components by giving them a unique id 
    /// on initialization for which you then search using this function. 
    /// 
    /// If a component with a matching id is found
    /// within the active components list (that is, if you've called `Engine::spawn(cpy!(/* Component */))?` (see: [spawn](Engine::spawn))
    /// on the component with the searched-for id), it will be returned. Otherwise, [find](Commands::find)
    /// will throw a [RuntimeException].
    /// ## Example
    /// ```
    /// fn __start__() -> Result<(), RuntimeException> {
    ///     let label = make!(Label::new("Label1", "Hello, World!", 100, 100));
    ///     Engine::spawn(label)?;
    /// }
    /// 
    /// fn __update__(&mut commands: Commands) {
    ///     let label = Engine::find("Label1");
    ///     assert_eq!(borrow!(label).text == "Hello, World!".to_string());
    /// }
    /// ```
    pub fn find<T: Component + 'static>(id: &str) -> RefMut<'_, T> {
        Engine::with_commands_static(|cmds| {
            cmds.find::<T>(id)
        }).unwrap().expect("[PINE] ACTOR NOT FOUND: actor was not found.")
    }

    pub fn find_as_ptr<T: Component + 'static>(id: &str) -> Option<Rc<RefCell<T>>> {
        Engine::with_commands_static(|cmds| {
            cmds.find_as_ptr::<T>(id)
        }).unwrap()
    }

    pub fn get_actor(id: &str) -> Result<Rc<RefCell<Actor>>, RuntimeException> {
        let actor = Engine::find_as_ptr::<Actor>(id);
        actor.into_result_value(format!("Actor '{id}' not found"))
    }

    /// ## Description
    /// Removes an active component from the list of active components, effectively destroying it and removing it
    /// from the game scene.
    /// ## Example
    /// ```
    /// fn on_start() -> Result<(), RuntimeException> {
    ///     let actor = make!(Actor::new("My Actor", ""));
    ///     Engine::spawn(actor.clone())?;
    ///     
    ///     if actor.get_texture().is_none() {
    ///         Engine::destroy(actor)?;
    ///     }
    /// }
    /// ```
    pub fn destroy(id: &'static str) -> Result<(), RuntimeException> {
        Engine::with_commands(|cmds| {
            cmds.destroy(id);
        }).into_result_string(format!("Failed to destroy component '{}'", id))
    }

    pub fn delete_global_var(id: &'static str) {
        Engine::with_commands(|cmds| {
            cmds.delete_global_var(id);
        });
    }

    /// ## Description
    /// Creates an active instance of a Component within your application by adding it to the list of active components.
    /// The [Commands] struct keeps track of all components currently active and 
    /// 1. tries to initialize them on [spawn](Engine::spawn) by adding it to the [Handle]
    /// 2. tries to render them (if renderable) every tick by using the [Renderer]
    /// The [spawn](Engine::spawn) method is used for the first step but leads consequently to the second, because
    /// on initialization, the component is added to the lsit of active components. The [update](Commands::update) method
    /// is automatically called each tick and calls each Components [render](Component::render) method for all the components
    /// within the active components.
    /// ```
    /// Engine::spawn(my_component)?;
    /// ``` 
    /// If you want your component to be renderable, you'll have to include that into the [render](Component::render) method of your component.
    /// Everything apart from rendering, that is, everything that does not need to be regenerated each tick goes into the
    /// [init](Component::init) method instead. 

    pub fn spawn(component: Rc<RefCell<dyn Component + 'static>>) ->  Result<(), RuntimeException> {
        Engine::with_commands(|cmds| {
            cmds.spawn(component.clone());
        }).into_result_string(format!("Failed to spawn component '{}'", component.borrow().component_id()))
    }

    /// ## Description
    /// Using [add_global_var](Engine::add_global_var) you can globalize local variables.
    /// Using this method (`get_global_var`) you can retrieve them back into scope.
    /// 
    /// Re-localized global vars are passed as `RefMut<'_, T>`, which means that they remain mutable
    /// throughout scopes. Changing the globalized variable in any scope means that it's contents will be
    /// mutated throughout scopes.
    /// ## Example
    /// ```
    /// fn start() -> Result<(), RuntimeException> {
    ///     let local_var = 0i32;
    ///     Engine::add_global_var("global_var", &local_var)?;
    /// }
    /// 
    /// fn on_mouse_button_down() -> Result<(), RuntimeException> {
    ///     let localized_var = Engine::get_global_var::<i32>("global_var");
    ///     *localized_var += 1;
    /// }
    /// ```
    pub fn get_global_var<T: Clone + 'static>(id: &str) -> RefMut<'_, T> {
        Engine::with_commands_static(|cmds| {
            cmds.get_global_var::<T>(id)
        }).unwrap().expect("[PINE] GLOBAL VARIABLE IS NULL: global variable does not exist.")
    }

    /// ## Description
    /// Using [add_global_var](Engine::add_global_var) you can globalize local variables.
    /// Use [get_global_var](Engine::get_global_var) to download them into your current scope.
    ///  
    /// Re-localized global vars are passed as `RefMut<'_, T>`, which means that they remain mutable
    /// throughout scopes. Changing the globalized variable in any scope means that it's contents will be
    /// mutated throughout scopes.
    /// ## Example
    /// ```
    /// fn start() -> Result<(), RuntimeException> {
    ///     let local_var = 0i32;
    ///     Engine::add_global_var("global_var", &local_var)?;
    /// }
    /// 
    /// fn on_mouse_button_down() -> Result<(), RuntimeException> {
    ///     let localized_var = Engine::get_global_var::<i32>("global_var");
    ///     *localized_var += 1;
    /// }
    /// ```
    pub fn add_global_var<T: Clone + 'static>(id: &str, var: &T) -> Result<(), RuntimeException> {
        Engine::with_commands_static(|cmds| {
            cmds.add_global_var(id, var);
        }).into_result_string(format!("Failed to add global variable '{}'", id))
    }

    pub(crate) fn set_active_commands(commands: Option<*mut Commands>) {
        ACTIVE_COMMANDS.with(|slot| {
            *slot.borrow_mut() = commands;
        });
    }

    /// ## Description
    /// Returns an instance of the currently active [Commands] instance as `&'static mut Commands` within an `no_commands` environment.
    /// ## Unsafe
    /// [Engine] has wrappers for almost all [Commands] functions. If you can, use them instead. Otherwise, check if you can use  
    /// [with_commands](Engine::with_commands) or [with_commands_static](Engine::with_commands_static). **Only** use this
    /// method if there is absolutely no other alternative. 
    pub unsafe fn get_active_commands() -> &'static mut Commands {
        ACTIVE_COMMANDS.with(|slot| {
            let ptr = (*slot.borrow()).unwrap();
            unsafe {
                ptr.as_mut().unwrap()
            }
        })
    }

    /// ## Description
    /// Creates a local scope, where [Commands] is borrowed (as `&'static mut Commands`).
    /// 
    /// [Engine] has wrappers for almost all [Commands] functions. If you can, use them instead.
    pub fn with_commands_static<R>(f: impl FnOnce(&'static mut Commands) -> R) -> Option<R> {
        let commands = unsafe { Engine::get_active_commands() };
        Some(f(commands))
    }

    /// ## Description
    /// Creates a local scope, where [Commands] is borrowed (as `&'static mut Commands`).
    /// 
    /// [Engine] has wrappers for almost all [Commands] functions. If you can, use them instead.
    /// ## Technical Info 
    /// Runs a closure with the currently active [`Commands`] context.
    ///
    /// This is useful in callbacks that intentionally avoid `&mut Commands`
    /// in their function signature, e.g. when using [`Window::new_no_commands`](crate::window::Window::new_no_commands)
    /// or `on_*_no_commands` hooks.
    ///
    /// Returns `None` when called outside of [`Window::run`](window::Window::run).
    pub fn with_commands<R>(f: impl FnOnce(&mut Commands) -> R) -> Option<R> {
        ACTIVE_COMMANDS.with(|slot| {
            let ptr = (*slot.borrow())?;
            // SAFETY: The pointer is only set by Window::run while the Commands value is alive.
            Some(unsafe { f(&mut *ptr) })
        })
    }
}



#[macro_export]
macro_rules! drop {
    ( $($e: expr),* ) => {
        $(
            drop($e);
        )*
    };
}

/// ## Description
/// [Components](Component) in Pine are passed wrapped in an `Rc<RefCell<Component>>`-Pointer structure.
/// Use the `make!` macro to easily create and wrap Components more efficiently **without exposing
/// too much pointer logic**.
/// ## Example
/// ```
/// fn __start__(commands: &mut Commands) {
///     let label = make!(Label::new("Label1", "Hello, World!", 100, 100));
///     // --snip--
/// }
/// ```
/// Usage always follows this structure:
/// ```
/// let component = make!(/* SomeComponent */);
/// ```
#[macro_export]
macro_rules! make {
    ($x:expr) => {
        Rc::new(RefCell::new($x))
    };
}

/// ## Description
/// The [try_find] macro is a shorthand to finding [Components](Component) in the list of currently active components that were 
/// created in a different function and try get an instance in the current function. [try_find] returns `Option<RefMut<'_, T>>`, with
/// `T` being the type of [Component] you are searching for.
/// ## Example
/// ```
/// fn start(commands: &mut Commands) {
///     let my_actor = make!(Actor::new("My Actor"));
///     commands.spawn(cpy!(my_actor)); // spawns component and adds it to the active components list 
/// }
/// 
/// fn update(commands: &mut Commands) {
///     let actor = try_find!(commands, Actor, "My Actor");
///     if actor.is_some() {
///         println!("Found actor!");
///     }
/// }
/// ```
/// The [try_find] macro is equivalent to the [find](Commands::find) method of [Commands]:
/// ```
/// let actor: Option<RefMut<'_, Actor>> = commands.find::<Actor>("My Actor");
/// ```
#[macro_export]
macro_rules! try_find {
    ($cmds:expr, $typeid:ty, $id:expr) => {
        $cmds.find::<$typeid>($id)
    };
}

/// ## Description
/// The [find] macro is a shorthand to finding [Components](Component) in the list of currently active components that were 
/// created in a different function and try get an instance in the current function. [find] returns `RefMut<'_, T>`, with
/// `T` being the type of [Component] you are searching for and [panics](panic) if the Component is not found.
/// ## Example
/// ```
/// fn start(commands: &mut Commands) {
///     let my_actor = make!(Actor::new("My Actor"));
///     commands.spawn(ptr!(my_actor)); // spawns component and adds it to the active components list 
/// }
/// 
/// fn update(commands: &mut Commands) {
///     let actor = find!(commands, Actor, "My Actor");
/// }
/// ```
/// The [find] macro is equivalent to the [find](Commands::find) method of [Commands]:
/// ```
/// let actor: RefMut<'_, Actor> = commands.find::<Actor>("My Actor").unwrap();
/// ```
#[macro_export]
macro_rules! find {
    ($cmds:expr, $typeid:ty, $id:expr) => {
        $cmds.find::<$typeid>($id).unwrap()
    };
}

/// ## Description
/// [Components](Component) in Pine are passed wrapped in an `Rc<RefCell<Component>>`-Pointer structure.
/// Use the `cpy!` macro to pass references to Components into functions that expect them as such, for example in the form of
/// - `&mut Rc<RefCell<dyn Component>>` 
/// - `&Rc<RefCell<dyn Component>>`
/// To create Components pre-wrapped in the required pointer structure, use the [make] macro. If you want to **modify the value of an component**, use the [get] macro.
/// ## Example
/// ```
/// fn __start__(commands: &mut Commands) {
///     let label = make!(Label::default());
///     commands.spawn(cpy!(label));
///     get!(label).text = "Hello, World".to_string(); // 'label' remains in this scope!
///     // --snip--
/// }
/// ```
/// ## Deprecated Macro
/// Just use `.clone()`, bro it ain't that deep.
#[deprecated]
#[macro_export]
macro_rules! cpy {
    ($x:expr) => {
        $x.clone()
    };
}

/// ## Description
/// [Components](Component) in Pine are passed wrapped in an `Rc<RefCell<Component>>`-Pointer structure.
/// Use the `get!` macro to borrow an instance of this Component pointer, **without exposing complex pointer structure**.
/// - When creating instances of Components, use the [make] macro.
/// - When passing references to Components to other functions, use the [cpy] macro.
/// ## Example
/// ```
/// fn __start__(&mut commands: Commands) {
///     let label = make!(Label::default());
///     get!(label).text = "Hello, World!";
/// }
/// ```
#[macro_export]
macro_rules! get {
    ($x:expr) => {
        $x.borrow_mut()
    };
}

/// ## Description
/// The [load] macro allows you to easily load a [global variable](Commands::global_variables) into the current scope.
/// It's a shorthand for the [get_global_var](Commands::get_global_var) method of [Commands]:
/// ```
/// let var = commands.get_global_var::<i32>("counter").unwrap();
/// ``` 
/// Or using the [Engine] wrapper:
/// ```
/// let var = Engine::get_global_var::<i32>("counter");
/// ```
/// Or using the `load!` macro:
/// ```
/// let var = load!(i32, "counter");
/// ```
/// The `load!` macro expands to the [Engine] definition:
/// ```
/// macro_rules! load {
///     ($typeid:ty, $id:expr) => {
///         Engine::get_global_var::<$typeid>($id)
///     }
/// }
/// ```
/// To upload a variable (**make a global variable**), use the [upload] macro.
/// ## Example
/// ```
/// fn start() -> Result<(), RuntimeError> {
///     let counter = 0u32;
///     upload!(&counter);
///     
///     Ok(())
/// }
/// 
/// fn update() -> Result<(), RuntimeError> {
///     let counter = load!(u32, "counter");
///     if counter < 10 {
///         *counter += 1;
///     }
/// 
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! load {
    ($typeid:ty, $id:expr) => {
        Engine::get_global_var::<$typeid>($id)
    };
}

/// ## Description
/// The [upload] macro allows you to easily upload a *local variable* to the [global variables](Commands::global_variables).
/// A **global variable** is a variable, that you can retrieve from another foreign scope using the [load] macro.
/// ## Example
/// ```
/// fn start() -> Result<(), RuntimeError> {
///     let counter = 0u32;
///     upload!("counter", &counter);
/// 
///     Ok(())
/// }
/// 
/// fn update() -> Result<(), RuntimeError> {
///     let counter = load!(u32, "counter");
///     if counter < 10 {
///         *counter += 1;
///     }
/// 
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! upload {
    ($id:expr, $var:expr) => {
        Engine::add_global_var($id, $var)
    };
}

use std::mem::MaybeUninit;

use sdl2::{sys::{SDL_CreateRenderer, SDL_CreateWindow, SDL_Event, SDL_INIT_VIDEO, SDL_Init, SDL_PollEvent, SDL_Renderer, SDL_Window}};
use sdl2::sys::{SDL_CreateTextureFromSurface, SDL_DestroyRenderer, SDL_DestroyWindow, SDL_EventType, SDL_FreeSurface, SDL_LoadBMP_RW, SDL_Quit, SDL_RWFromFile, SDL_RWops, SDL_Surface, SDL_Texture, image};

use crate::prelude::{Actor, Texture2D};
use crate::util::{ComponentSafecastRc, RuntimeException};
use crate::util::{AnySafecast, ComponentReferenceSafecast, IntoResult, cstr_rb};


/// ## Description
/// [Attributes](Attribute) are certain "traits" and "abilities" that [Components](Component) can expand on.
pub trait Attribute {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn start(&mut self) {}
    fn update(&mut self) {}
}

/// ## Description
/// The [Handle] is the back bone of the [Window](window::Window) struct. It *handles* all processes
/// designated to the displayed Frame. The Handle is part of the greater [Commands] struct.
/// ## Disclaimer
/// **Wrapper struct** for the actual [SDL_Window](sdl2::sys). Use the [get](Handle::get) function to retrieve the
/// actual sdl2 window instance as `*mut SDL_Window`.
#[derive(Debug, Clone)]
pub struct Handle(pub(in crate) *mut SDL_Window);

/// ## Description
/// The [Renderer] is used to render [Components](Component) to the [screen](window::Window). The Renderer is part of the greater
/// the [Commands].
/// ## Disclaimer
/// **Wrapper struct** for the actual [SDL_Renderer](sdl2::sys). Use the [get](Renderer::get) function to retrieve the
/// actual sdl2 renderer instance as `*mut SDL_Renderer`.
#[derive(Debug, Clone)]
pub struct Renderer {
    pub(in crate) renderer: *mut SDL_Renderer,
    pub world_scale: f32,
    pub logical_width: f32,
    pub logical_height: f32,
}

impl Renderer {
    pub(in crate) fn new(renderer: *mut SDL_Renderer, pixel_width: usize, pixel_height: usize) -> Self {
        Self {
            renderer,
            world_scale: 1.0,
            logical_width: pixel_width as f32,
            logical_height: pixel_height as f32,
        }
    }

    pub fn set_logical_size(&mut self, logical_w: f32, logical_h: f32, pixel_w: u32, pixel_h: u32) {
        self.logical_width = logical_w;
        self.logical_height = logical_h;

        let scale_x = pixel_w as f32 / self.logical_width as f32;
        let scale_y = pixel_h as f32 / self.logical_height as f32;

        self.world_scale = scale_x.min(scale_y);
    }

    pub(in crate) fn get(&self) -> *mut SDL_Renderer {
        self.renderer
    }

    pub(in crate) unsafe fn get_or_create_texture(&self, texture: &mut Texture2D) -> *mut SDL_Texture {
        if let Some(tex) = texture.sdl_texture {
            return tex;
        }

        let cstr_path = CString::new(texture.file_path.as_str()).unwrap();

        let rwops = SDL_RWFromFile(cstr_path.as_ptr(), cstr_rb());
        if rwops.is_null() {
            panic!("RENDERER: Failed to open texture file: {}", texture.get());
        }

        let surface = image::IMG_Load_RW(rwops, 1);
        if surface.is_null() {
            panic!("RENDERER: Failed to load BMP surface from file: {}", texture.get());
        }

        let tex = SDL_CreateTextureFromSurface(self.get(), surface);
        if tex.is_null() {
            panic!("RENDERER: Failed to create SDL_Texture from surface: {}", texture.get());
        }

        SDL_FreeSurface(surface);

        let (w, h) = Texture2D::query_texture_size(tex);

        texture.width = w;
        texture.height = h;
        texture.sdl_texture = Some(tex);
        tex
    }

    #[deprecated]
    pub(in crate) unsafe fn texture_from_asset(&self, texture: &Texture2D) -> *mut SDL_Texture {
        let cstr_path = CString::new(texture.get()).unwrap();

        let rwops: *mut SDL_RWops = SDL_RWFromFile(cstr_path.as_ptr(), CString::new("rb").unwrap().as_ptr());
        if rwops.is_null() {
            panic!("RENDERER: Failed to open texture file: {}", texture.get());
        }

        let surface = sdl2::sys::image::IMG_Load_RW(rwops, 1);
        if surface.is_null() {
            panic!("RENDERER: Failed to load BMP surface from file: {}", texture.get());
        }

        let sdl_texture: *mut SDL_Texture = SDL_CreateTextureFromSurface(self.get(), surface);
        if sdl_texture.is_null() {
            panic!("RENDERER: Failed to create SDL_Texture from surface: {}", texture.get());
        }

        SDL_FreeSurface(surface);

        sdl_texture
    }
}

impl Handle {
    pub(in crate) fn get(&self) -> *mut SDL_Window {
        self.0
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// ## Description
/// **Components** are the most important unit of your game. Every game is made up of a set of components. The components
/// are stored within the [Commands] control struct. Create components in your games [start](window::Window::start)-callback.
/// 
/// If you want your component to be renderable, you'll have to include that into the [render](Component::render) method of your component.
/// Everything apart from rendering, that is, everything that does not need to be regenerated each tick goes into the
/// [init](Component::init) method instead. 
/// 
/// The most important Component is the [Actor](basic_components::Actor) component, which is special in that it can take
/// [Attributes](crate::Attribute) and affect how it acts in real time in the game world. 
/// ## Super Components
/// A **Super Component** is a wrapper struct around an [Component] (most of the time around [Actor](basic_components::Actor)) that
/// takes the original component and builds on top of it (e.g. for Actor, include some attributes thus creating an **Actor blueprint**).
/// ### Example
/// ```
/// pub struct Card(pub Rc<RefCell<Actor>>);
/// 
/// impl Card {
///     pub fn new(id: &str, texture_path: &str, rank: Rank, suit: Suit) -> Card {
///         let base_component = make!(Actor::new(id, texture_path));
///         get!(base_component).add_attribute(rank, format!("{}_rank", id));
///         get!(base_component).add_attribute(suit, format!("{}_suit", id));
/// 
///         Card(base_component)
///     }
/// }
/// ```
/// Here, `Card` is a super component for `Actor` that includes the custom defined `Rank` and `Suit` attributes.
pub trait Component: std::any::Any {
    /// ## Description
    /// If you want your component to be renderable, you'll have to include that into the [render](Component::render) method of your component.
    /// Everything apart from rendering, that is, everything that does not need to be regenerated each tick goes into the
    /// [init](Component::init) method instead. 
    fn init(&self, handle: &mut Handle);
    
    /// ## Description
    /// This returns the unique identifier of this [Component], usually, you'll let this be set by the user on initialization,
    /// or you automatically assing a name based on a standardized naming convention.
    /// 
    /// **Caution!**: When your component has no proper component id, you won't be able to find it using [Commands::find]!
    fn component_id(&self) -> String;
    fn component_type(&self) -> String {
        String::from("Basic Component")
    }
    
    /// ## Description
    /// If you want your component to be renderable, you'll have to include that into the [render](Component::render) method of your component.
    /// Everything apart from rendering, that is, everything that does not need to be regenerated each tick goes into the
    /// [init](Component::init) method instead. 
    fn render(&mut self, renderer: &mut Renderer);

    fn clone_box(&self) -> Box<dyn Component>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn get_attributes(&self) -> Vec<(String, Rc<RefCell<dyn Attribute>>)>;
}

/// ## Discription
/// The [Commands] struct provides the necessary tools for controlling your application. Each application consists of
/// a [Window](window::Window) which is assigned both a [start](window::Window::start) and an [update](window::Window::update) method. (for more, see: [Window](window::Window)).
/// 
/// Inside of the [start](window::Window::start) and [update](window::Window::update) method, you can access the [Commands] struct
/// to modify the application. Some important methods are:
/// - [spawn](Commands::spawn): use `spawn` to create new instances of Components
/// - [find](Commands::find): use `find` to retrieve an active object by searching for it's unique identifier
///
/// The [Commands] struct keeps track of all components currently active and:
/// 1. tries to initialize them on [spawn](Commands::spawn) by adding it to the [Handle]
/// 2. tries to render them (if renderable) every tick by using the [Renderer]
/// ## Example
/// ```
/// fn main() {
///     let window = Window::new("My Window", __start__, __update__);
///     window.run(); // starts the program
/// }
/// 
/// fn __start__(&mut commands: Commands) {
///     let label = make!(Label::new("Label1", "Hello, World!", 100, 100));
///     commands.spawn(cpy!(label)); // add label to active components
/// }
/// 
/// fn __update__(&mut commands: Commands) {
///     let label = commands.find("Label1").unwrap(); // find active component "Label1"
///     println!("found label1 with text: {}", label.text);
/// }
/// ```
#[derive(Clone)]
pub struct Commands {
    pub handle: Handle,
    pub renderer: Renderer,
    pub active_components: Vec<ComponentPointer>,
    pub window_bounds: (i32, i32),
    pub world_size: (f32, f32),
    pub global_variables: Vec<(String, Rc<RefCell<dyn std::any::Any>>)>,
}

impl Commands {
    /// ## Description
    /// Creates an active instance of a Component within your application by adding it to the list of active components.
    /// The [Commands] struct keeps track of all components currently active and 
    /// 1. tries to initialize them on [spawn](Commands::spawn) by adding it to the [Handle]
    /// 2. tries to render them (if renderable) every tick by using the [Renderer]
    /// The [spawn](Commands::spawn) method is used for the first step but leads consequently to the second, because
    /// on initialization, the component is added to the lsit of active components. The [update](Commands::update) method
    /// is automatically called each tick and calls each Components [render](Component::render) method for all the components
    /// within the active components.
    /// 
    /// If you want your component to be renderable, you'll have to include that into the [render](Component::render) method of your component.
    /// Everything apart from rendering, that is, everything that does not need to be regenerated each tick goes into the
    /// [init](Component::init) method instead. 
    pub fn spawn(&mut self, component: ComponentPointer) {
        component.borrow().init(&mut self.handle);

        // initialize all attributes
        let attributes = component.borrow().get_attributes();
        if attributes.len() > 0 {
            for attr in attributes {
                attr.1.borrow_mut().start();
            }
        }

        self.active_components.push(component);
    }

    /// ## Description
    /// Removes an active component from the list of active components, effectively destroying it and removing it
    /// from the game scene.
    /// ## Example
    /// ```
    /// fn on_start(commands: &mut Commands) {
    ///     let actor = make!(Actor::new("My Actor", ""));
    ///     commands.spawn(actor);
    ///     
    ///     if actor.get_texture().is_none() {
    ///         commands.destroy("My Actor");
    ///     }
    /// }
    /// ```
    pub fn destroy(&mut self, id: impl Into<String> + Clone) {
        let idx = self.active_components
            .iter()
            .position(|x| x.borrow().component_id() == id.clone().into());

        if let Some(position) = idx {
            self.active_components.remove(position);
        }
    }

    pub fn delete_global_var(&mut self, id: &str) {
        for (idx, var) in self.global_variables.clone().iter().enumerate() {
            if var.0 == id.to_string() {
                self.global_variables.remove(idx);
            }
        }
    }

    /// ## Description
    /// The [update](Commands::update) method
    /// is automatically called each tick and calls each Components [render](Component::render) method for all the components
    /// within the active components. For more info, [click here](Commands::spawn).
    pub fn update(&mut self, component: ComponentPointer) {
        component.borrow_mut().render(&mut self.renderer);

        // update all attributes
        let attrs = component.borrow().get_attributes();
        if attrs.len() > 0 {
            for attr in attrs {
                attr.1.borrow_mut().update();
            }
        }
    }    

    /// ## Description
    /// The [find](Commands::find) method of the [Commands] struct allows you to search for currently active
    /// [Components](Component) and return them if found. You search for Components by giving them a unique id 
    /// on initialization for which you then search using this function. 
    /// 
    /// If a component with a matching id is found
    /// within the active components list (that is, if you've called `commands.spawn(cpy!(/* Component */))` (see: [spawn](Commands::spawn))
    /// on the component with the searched-for id), it will be returned as `Some(/* Component */)`. Otherwise, [find](Commands::find) will
    /// return `None`.
    /// ## Example
    /// ```
    /// fn __start__(&mut commands: Commands) {
    ///     let label = make!(Label::new("Label1", "Hello, World!", 100, 100));
    ///     commands.spawn(cpy!(label));
    /// }
    /// 
    /// fn __update__(&mut commands: Commands) {
    ///     let label = commands.find("Label1").unwrap();
    ///     assert_eq!(borrow!(label).text == "Hello, World!".to_string());
    /// }
    /// ```
    pub fn find<T: Component + 'static>(&mut self, id: &str) -> Option<RefMut<'_, T>> {
        let index = self.active_components
            .iter()
            .position(|x| {
                x.borrow().component_id() == id
            })?;

        self.active_components[index].safecast_ref::<T>()
    }

    pub fn find_as_ptr<T: Component + 'static>(&self, id: &str) -> Option<Rc<RefCell<T>>> {
        let component = self.active_components
            .iter()
            .find(|x| x.borrow().component_id() == id)?;

        component.clone().safecast_rc::<T>()
    }

    pub fn add_global_var<T: std::any::Any + Clone + 'static>(&mut self, id: impl Into<String>, var: &T) {
        self.global_variables.push((id.into(), Rc::new(RefCell::new(var.clone()))));
    }

    pub fn get_global_var<T: std::any::Any + 'static>(&mut self, id: impl Into<String> + Clone) -> Option<RefMut<'_, T>> {
        let index = self.global_variables
            .iter()
            .position(|x| x.0 == id.clone().into())?;

        self.global_variables[index].1.safecast_ref::<T>()
    }

    pub fn width(&self) -> i32 {
        self.window_bounds.0
    }

    pub fn height(&self) -> i32 {
        self.window_bounds.1
    }

    pub fn half_width(&self) -> i32 {
        (self.window_bounds.0 / 2) as i32
    }

    pub fn half_height(&self) -> i32 {
        (self.window_bounds.1 / 2) as i32
    }

    pub fn center(&self) -> (i32, i32) {
        (self.half_width(), self.half_height())
    }

    pub fn set_logical_size(&mut self, logical_w: f32, logical_h: f32) {
        let pixel_w = self.width() as u32;
        let pixel_h = self.height() as u32;
        
        self.renderer.set_logical_size(logical_w, logical_h, pixel_w, pixel_h);
    }
}