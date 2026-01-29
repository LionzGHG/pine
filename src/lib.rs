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
//! For further orientation, get yourself acquainted with the [Window](window::Window), [Component] and [Commands] structs.  

pub type ComponentPointer = Rc<RefCell<dyn Component>>;

pub mod basic_components;
pub mod window;
pub mod prelude;
pub mod util;
pub mod assets;
pub mod basic_attributes;

use std::cell::RefCell;
use std::rc::Rc;

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
/// [Components](Component) in Pine are passed wrapped in an `Rc<RefCell<Component>>`-Pointer structure.
/// Use the `cpy!` macro to pass references to Components into functions that expect them as such, for example in the form of
/// - `&mut Rc<RefCell<dyn Component>>` 
/// - `&Rc<RefCell<dyn Component>>`
/// To create Components pre-wrapped in the required pointer structure, use the [make] macro. If you want to **modify the value of an component**, use the [qry] macro.
/// ## Example
/// ```
/// fn __start__(commands: &mut Commands) {
///     let label = make!(Label::default());
///     commands.spawn(cpy!(label));
///     qry!(label).text = "Hello, World".to_string(); // 'label' remains in this scope!
///     // --snip--
/// }
/// ```
#[macro_export]
macro_rules! cpy {
    ($x:expr) => {
        $x.clone()
    };
}

/// ## Description
/// [Components](Component) in Pine are passed wrapped in an `Rc<RefCell<Component>>`-Pointer structure.
/// Use the `qry!` macro to borrow an instance of this Component pointer, **without exposing complex pointer structure**.
/// - When creating instances of Components, use the [make] macro.
/// - When passing references to Components to other functions, use the [cpy] macro.
/// ## Example
/// ```
/// fn __start__(&mut commands: Commands) {
///     let label = make!(Label::default());
///     qry!(label).text = "Hello, World!";
/// }
/// ```
#[macro_export]
macro_rules! qry {
    ($x:expr) => {
        $x.borrow_mut()
    };
}

use std::{mem::MaybeUninit};

use sdl2::{sys::{SDL_CreateRenderer, SDL_CreateWindow, SDL_Event, SDL_INIT_VIDEO, SDL_Init, SDL_PollEvent, SDL_Renderer, SDL_Window}};
use sdl2::sys::{SDL_DestroyRenderer, SDL_DestroyWindow, SDL_EventType, SDL_Quit};

use crate::assets::{AsAny, Safecast};


/// ## Description
/// [Attributes](Attribute) are certain "traits" and "abilities" that [Components](Component) can expand on.
pub trait Attribute {}

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
}

impl Handle {
    pub(in crate) fn get(&self) -> *mut SDL_Window {
        self.0
    }
}

pub trait Component {
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
pub struct Commands {
    pub handle: Handle,
    pub renderer: Renderer,
    pub active_components: Vec<ComponentPointer>,
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
    ///     assert_eq!(qry!(label).text == "Hello, World!".to_string());
    /// }
    /// ```
    pub fn find(&self, id: &str) -> Option<&ComponentPointer> {
        self.active_components.iter()
            .find(|x| qry!(x).component_id() == id.to_string())
    }
}