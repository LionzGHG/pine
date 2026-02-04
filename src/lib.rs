//! # Welcome to Pine!
//! ***Pine*** is a small, easy to use 2d game engine for rust. To start your application, orient yourself according
//! to this setup guide!
//! ## Setup-Guide
//! To start with your application, follow this general application layout:
//! ```
//! fn main() {
//!     let window = Window::new("My Window", __start__, __update__);
//!     window.run();
//! }
//! 
//! fn __start__(&mut commands: Commands) {
//!     println!("Program has started!");
//! }
//! 
//! fn __update__(&mut commands: Commands) {
//!     println!("Program is running!");
//! }
//! ```
//! For further orientation, get yourself acquainted with the [Window](window::Window), [Component] and [Commands] structs, as well as
//! the [Actor](basic_components::Actor) [Component]. 
//! ## Minimal Example
//! Below is a minimal example of a game, where you can control a player.
//! 
//! **Notice**: You'll have to place an `.png` or `.jpg` file at `./assets/textures/player`!
//! ```
//! use pine::prelude::*;
//! 
//! fn main() {
//!     let mut window = Window::new("My Window", start, update);
//!     window.on_key_down(on_key_down);
//!     window.run();
//! }
//! 
//! pub fn on_key_down(commands: &mut Commands, keycode: i32) {
//!     let mut player = find!(commands, Actor, "Player");
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
//! pub fn start(commands: &mut Commands) {
//!     let player = make!(Actor::new("Player", "player"));
//!     commands.spawn(cpy!(player));
//! 
//! } 
//! 
//! pub fn update(commands: &mut Commands) {}
//! ``` 

pub type ComponentPointer = Rc<RefCell<dyn Component>>;

pub mod basic_components;
pub mod window;
pub mod prelude;
pub mod util;
pub mod assets;
pub mod basic_attributes;
pub mod math;

use std::cell::{RefCell, RefMut};
use std::ffi::CString;
use std::rc::Rc;

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
/// Or using the `load!` macro:
/// ```
/// let var = load!(commands, i32, "counter");
/// ```
/// To upload a variable (**make a global variable**), use the [upload] macro.
/// ## Example
/// ```
/// fn start(commands: &mut Commands) {
///     let counter = 0u32;
///     upload!(commands, &counter);
/// }
/// 
/// fn update(commands: &mut Commands) {
///     let counter = load!(commands, u32, "counter");
///     if counter < 10 {
///         *counter += 1;
///     }
/// }
/// ```
#[macro_export]
macro_rules! load {
    ($cmds:expr, $typeid:ty, $id:expr) => {
        $cmds.get_global_var::<$typeid>($id).unwrap()
    };
}

/// ## Description
/// The [upload] macro allows you to easily upload a *local variable* to the [global variables](Commands::global_variables).
/// A **global variable** is a variable, that you can retrieve from another foreign scope using the [load] macro.
/// ## Example
/// ```
/// fn start(commands: &mut Commands) {
///     let counter = 0u32;
///     upload!(commands, &counter);
/// }
/// 
/// fn update(commands: &mut Commands) {
///     let counter = load!(commands, u32, "counter");
///     if counter < 10 {
///         *counter += 1;
///     }
/// }
/// ```
#[macro_export]
macro_rules! upload {
    ($cmds:expr, $var:expr) => {
        $cmds.add_global_var(stringify!($var), $var);
    };
}

use std::{mem::MaybeUninit};

use sdl2::{sys::{SDL_CreateRenderer, SDL_CreateWindow, SDL_Event, SDL_INIT_VIDEO, SDL_Init, SDL_PollEvent, SDL_Renderer, SDL_Window}};
use sdl2::sys::{SDL_CreateTextureFromSurface, SDL_DestroyRenderer, SDL_DestroyWindow, SDL_EventType, SDL_FreeSurface, SDL_LoadBMP_RW, SDL_Quit, SDL_RWFromFile, SDL_RWops, SDL_Surface, SDL_Texture, image};

use crate::prelude::Texture2D;
use crate::util::{AnySafecast, ComponentReferenceSafecast, cstr_rb};


/// ## Description
/// [Attributes](Attribute) are certain "traits" and "abilities" that [Components](Component) can expand on.
pub trait Attribute {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
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
pub struct Renderer(pub(in crate) *mut SDL_Renderer);

impl Renderer {
    pub(in crate) fn get(&self) -> *mut SDL_Renderer {
        self.0
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
    
    /// ## Description
    /// If you want your component to be renderable, you'll have to include that into the [render](Component::render) method of your component.
    /// Everything apart from rendering, that is, everything that does not need to be regenerated each tick goes into the
    /// [init](Component::init) method instead. 
    fn render(&mut self, renderer: &mut Renderer);

    fn clone_box(&self) -> Box<dyn Component>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
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
        self.active_components.push(component);
    }

    /// ## Description
    /// The [update](Commands::update) method
    /// is automatically called each tick and calls each Components [render](Component::render) method for all the components
    /// within the active components. For more info, [click here](Commands::spawn).
    pub fn update(&mut self, component: ComponentPointer) {
        component.borrow_mut().render(&mut self.renderer);
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
            .position(|x| x.borrow().component_id() == id)?;

        self.active_components[index].safecast_ref::<T>()
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
}