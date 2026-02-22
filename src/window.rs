use std::ffi::CString;
use std::mem::MaybeUninit;
use std::time::Instant;

use crate::math::Point;
use crate::util::{RuntimeException, Time};
use crate::{Commands, Handle, Renderer, Engine};

use sdl2::sys::{SDL_CreateRenderer, SDL_CreateWindow, SDL_Event, SDL_GetHint, SDL_HINT_RENDER_VSYNC, SDL_INIT_VIDEO, SDL_Init, SDL_PollEvent, SDL_RenderClear, SDL_RenderPresent, SDL_RenderSetLogicalSize, SDL_SetHint, SDL_SetRenderDrawColor, SDL_Window};
use sdl2::sys::{SDL_DestroyRenderer, SDL_DestroyWindow, SDL_EventType, SDL_Quit};

const DEFAULT_TARGET_FPS: f32 = 60.0;
const DEFAULT_TARGET_FRAME_TIME: f32 = 1.0 / DEFAULT_TARGET_FPS;

pub(in crate) fn compute_target_frame_time_from_target_fps(target_fps: f32) -> f32 {
    1.0 / target_fps 
}

/// ## Description
/// The [Window] struct is the starting point of your application. You can create a basic window by using the
/// [new](Window::new) method, which expects:
/// - `title`: The title of your window, displayed at the top left
/// - `on_start`: A method, that runs once at application start-up, following this scheme: `fn __start__(&mut commands: Commands) {}`
/// - `on_tick`: A method, that runs once every tick, following this scheme: `fn __update__(&mut commands: Commands) {}` 
/// If you want to tweak additional parameters, you can access the other fields of the [Window] struct:
/// - `pos_x`: The x-position of the window
/// - `pos_y`: The y-position of the window
/// - `width`: The width of the window
/// - `height`: the height of the window
/// 
/// Additionally, you can connect an assortment of other methods to possible events, those are:
/// - `on_key_down`: A function that runs, whenever a key is pressed (click [here](Window::on_key_down) for further information)
/// The field `running` tells you, whether the window is currently running or not.
/// 
/// Use the [run](Window::run) method to tell the program that it should start running.
/// 
/// You can create a simple example window like this:
/// ```
/// fn main() {
///     let window = Window::new("My Window", __start__, __update__);
///     window.run();
/// }
/// 
/// fn __start__(&mut commands: Commands) {
///     println!("Program has started!");
/// }
/// 
/// fn __update__(&mut commands: Commands) {
///     println!("Program is running!");
/// }
/// ```
pub struct Window {
    pub title: String,
    pub pos_x: usize,
    pub pos_y: usize,
    pub width: usize,
    pub height: usize,
    pub logical_width: usize,
    pub logical_height: usize,
    pub(in crate) handle: Handle,
    pub(in crate) renderer: Renderer,
    pub running: bool,

    pub target_fps: f32,
    pub target_frame_time: f32,

    // Event Handling
    pub start: Option<Box<dyn Fn(&mut Commands) -> Result<(), RuntimeException>>>,
    pub update: Option<Box<dyn Fn(&mut Commands) -> Result<(), RuntimeException>>>,
    pub start_no_commands: Option<Box<dyn Fn() -> Result<(), RuntimeException>>>,
    pub update_no_commands: Option<Box<dyn Fn() -> Result<(), RuntimeException>>>,

    pub on_key_down: Option<Box<dyn Fn(&mut Commands, i32) -> Result<(), RuntimeException>>>,
    pub on_mouse_button_down: Option<Box<dyn Fn(&mut Commands, i32, Point) -> Result<(), RuntimeException>>>,
    pub on_mouse_motion: Option<Box<dyn Fn(&mut Commands, Point) -> Result<(), RuntimeException>>>,

    pub on_key_down_no_commands: Option<Box<dyn Fn(i32) -> Result<(), RuntimeException>>>,
    pub on_mouse_button_down_no_commands: Option<Box<dyn Fn(i32, Point) -> Result<(), RuntimeException>>>,
    pub on_mouse_motion_no_commands: Option<Box<dyn Fn(Point) -> Result<(), RuntimeException>>>,
}

impl Window {
    pub fn new(title: &str, on_start: impl Fn(&mut Commands) -> Result<(), RuntimeException> + 'static, on_tick: impl Fn(&mut Commands) -> Result<(), RuntimeException> + 'static) -> Window {
        unsafe {
            SDL_Init(SDL_INIT_VIDEO);

            let name = CString::new("SDL_RENDER_VSYNC").unwrap();
            let hint = CString::new("1").unwrap();
            SDL_SetHint(name.as_ptr(), hint.as_ptr());
            println!("VSync hint: {}", SDL_GetHint(name.as_ptr()).is_null());

            let c_title = CString::new(title).unwrap();

            let raw_handle: *mut SDL_Window = SDL_CreateWindow(c_title.as_ptr(), 400, 400, 800, 600, 0);

            Window {
                title: title.to_string().clone(),
                pos_x: 0,
                pos_y: 0,
                width: 800,
                height: 600,
                logical_width: 800,
                logical_height: 600,
                handle: Handle(raw_handle),
                renderer: Renderer::new(SDL_CreateRenderer(raw_handle, -1, 0), 800, 600),
                running: false,
                target_fps: DEFAULT_TARGET_FPS,
                target_frame_time: DEFAULT_TARGET_FRAME_TIME,
                start: Some(Box::new(on_start)),
                update: Some(Box::new(on_tick)),
                start_no_commands: None,
                update_no_commands: None,
                on_key_down: None,
                on_mouse_button_down: None,
                on_mouse_motion: None,
                on_key_down_no_commands: None,
                on_mouse_button_down_no_commands: None,
                on_mouse_motion_no_commands: None
            }
        }   
    }

    pub fn new_no_commands(title: &str, on_start: impl Fn() -> Result<(), RuntimeException> + 'static, on_tick: impl Fn() -> Result<(), RuntimeException> + 'static) -> Window {
        unsafe {
            SDL_Init(SDL_INIT_VIDEO);

            let name = CString::new("SDL_RENDER_VSYNC").unwrap();
            let hint = CString::new("1").unwrap();
            SDL_SetHint(name.as_ptr(), hint.as_ptr());
            println!("VSync hint: {}", SDL_GetHint(name.as_ptr()).is_null());

            let c_title = CString::new(title).unwrap();

            let raw_handle: *mut SDL_Window =
                SDL_CreateWindow(c_title.as_ptr(), 400, 400, 800, 600, 0);

            Window {
                title: title.to_string().clone(),
                pos_x: 0,
                pos_y: 0,
                width: 800,
                height: 600,
                logical_width: 800,
                logical_height: 600,
                handle: Handle(raw_handle),
                renderer: Renderer::new(SDL_CreateRenderer(raw_handle, -1, 0), 800, 600),
                running: false,
                target_fps: DEFAULT_TARGET_FPS,
                target_frame_time: DEFAULT_TARGET_FRAME_TIME,
                start: None,
                update: None,
                start_no_commands: Some(Box::new(on_start)),
                update_no_commands: Some(Box::new(on_tick)),
                on_key_down: None,
                on_key_down_no_commands: None,
                on_mouse_button_down: None,
                on_mouse_button_down_no_commands: None,
                on_mouse_motion: None,
                on_mouse_motion_no_commands: None,
            }
        }
    }

    pub unsafe fn set_target_fps(&mut self, target_fps: f32) {
        self.target_fps = target_fps;
        self.target_frame_time = compute_target_frame_time_from_target_fps(target_fps);
    }

    pub fn set_logical_size(&mut self, width: usize, height: usize) {
        self.renderer.logical_width = width as f32;
        self.renderer.logical_height = height as f32;

        let pixel_w = self.width as f32;
        let pixel_h = self.height as f32;

        let scale_x = pixel_w / self.logical_width as f32;
        let scale_y = pixel_h / self.logical_height as f32;

        self.renderer.world_scale = scale_x.min(scale_y);
    }

    pub fn logical_size(&self) -> (usize, usize) {
        (self.logical_width, self.logical_height)
    }

    /// ## Description
    /// `on_key_down` allows you to specify a method that runs every time a key press is performed and recognized.
    /// 
    /// The **key down callback function** allows you to access parameters such as [Commands] and the `keycode` of
    /// the pressed key. You can use `if` to compare this `i32`-value to an existing [KeyCode](crate::util::KeyCode).
    /// ## Example
    /// ```
    /// fn main() {
    ///     let mut window = Window::new("My Window", __start__, __update__);
    ///     window.on_key_down(__key_callback__);
    ///        
    ///     window.run();
    /// }
    /// 
    /// fn __key_callback__(_commands: &mut Commands, keycode: i32) {
    ///     if keycode == KeyCode::SPACE {
    ///         println!("Spacebar pressed!")
    ///     }
    /// }
    /// ```
    pub fn on_key_down<F: Fn(&mut Commands, i32) -> Result<(), RuntimeException> + 'static>(&mut self, f: F) {
        self.on_key_down = Some(Box::new(f));
    }

    /// ## Description
    /// [`on_key_down_no_commands`](Window::on_key_down_no_commands) allows you to specify a method that runs every time a key press is performed and recognized.
    /// 
    /// The **key down callback function** allows you to access the `keycode` of
    /// the pressed key. You can use `if` to compare this `i32`-value to an existing [KeyCode](crate::util::KeyCode).
    /// ## Example
    /// ```
    /// fn main() {
    ///     let mut window = Window::new_no_commands("My Window", __start__, __update__);
    ///     window.on_key_down_no_commands(__key_callback__);
    ///        
    ///     window.run();
    /// }
    /// 
    /// fn __key_callback__(keycode: i32) -> Result<(), RuntimeException> {
    ///     if keycode == KeyCode::SPACE {
    ///         println!("Spacebar pressed!")
    ///     }
    /// }
    /// ```
    pub fn on_key_down_no_commands<F: Fn(i32) -> Result<(), RuntimeException> + 'static>(&mut self, f: F) {
        self.on_key_down_no_commands = Some(Box::new(f));
    }

    pub fn on_mouse_button_down<F: Fn(&mut Commands, i32, Point) -> Result<(), RuntimeException> + 'static>(&mut self, f: F) {
        self.on_mouse_button_down = Some(Box::new(f));
    }

    pub fn on_mouse_button_down_no_commands<F: Fn(i32, Point) -> Result<(), RuntimeException> + 'static>(&mut self, f: F) {
        self.on_mouse_button_down_no_commands = Some(Box::new(f));
    }

    pub fn on_mouse_motion<F: Fn(&mut Commands, Point) -> Result<(), RuntimeException> + 'static>(&mut self, f: F) {
        self.on_mouse_motion = Some(Box::new(f));
    }

    pub fn on_mouse_motion_no_commands<F: Fn(Point) -> Result<(), RuntimeException> + 'static>(&mut self, f: F) {
        self.on_mouse_motion_no_commands = Some(Box::new(f));
    }

    /// ## Description
    /// Override the **bounds** of your window by providing `width`, `height`, `x` position and `y` position.
    /// 
    /// By default, these values are set to `width=800`, `height=600` and `x=y=100`.
    pub fn set_bounds(&mut self, width: usize, height: usize, x: usize, y: usize) {
        self.width = width;
        self.height = height;
        self.pos_x = x;
        self.pos_y = y;

        unsafe {
            SDL_DestroyWindow(self.handle.0);
            SDL_DestroyRenderer(self.renderer.get());

            let new_raw_handle: *mut SDL_Window = SDL_CreateWindow(
                self.title.as_ptr() as *const i8,
                self.pos_x as i32,
                self.pos_y as i32,
                self.width as i32,
                self.height as i32,
                0,
            );

            self.handle = Handle(new_raw_handle);
            self.renderer = Renderer::new(SDL_CreateRenderer(new_raw_handle, -1, 0), width, height)    
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        self.set_logical_size(self.logical_width, self.logical_height);

        let mut cmds = Commands {
            handle: self.handle.clone(),
            renderer: self.renderer.clone(),
            active_components: Vec::new(),
            window_bounds: (self.width as i32, self.height as i32),
            world_size: (self.renderer.logical_width, self.renderer.logical_height),
            global_variables: Vec::new(),
        };

        println!("world_scale={}", self.renderer.world_scale);

        Engine::set_active_commands(Some(&mut cmds as *mut Commands));

        unsafe {
            SDL_RenderSetLogicalSize(
                self.renderer.get(),
                self.logical_width as i32,
                self.logical_height as i32
            );
        }

        unsafe {
            SDL_SetHint(
                CString::new("SDL_RENDER_SCALE_QUALITY").unwrap().as_ptr(),
                CString::new("2").unwrap().as_ptr()
            );
        }

        let mut error_stack = Vec::<RuntimeException>::new();

        // Call start procedure
        if let Some(start) = &self.start {
            let err = start(&mut cmds);
            if let Err(re) = err {
                error_stack.push(re);
            } 
        }
        if let Some(start) = &self.start_no_commands {
            let err = start();
            if let Err(re) = err {
                error_stack.push(re);
            }
        }

        // event handling
        let mut event = MaybeUninit::<SDL_Event>::uninit();

        Time::init();
        while self.running {
            let frame_start = Instant::now();

            // Events
            unsafe {
                while SDL_PollEvent(event.as_mut_ptr()) == 1 {
                    let event = event.assume_init();

                    // Handle on_quit event
                    if event.type_ == SDL_EventType::SDL_QUIT as u32 {
                        self.running = false;
                    }

                    // Handle on_key_down event
                    if event.type_ == SDL_EventType::SDL_KEYDOWN as u32 {
                        if let Some(func) = &self.on_key_down {
                            let err = func(&mut cmds, event.key.keysym.sym);
                            if let Err(re) = err {
                                error_stack.push(re);
                            }
                        }

                        if let Some(func) = &self.on_key_down_no_commands {
                            let err = func(event.key.keysym.sym);
                            if let Err(re) = err {
                                error_stack.push(re);
                            }
                        }
                    }

                    // Handle mouse_button_down event
                    if event.type_ == SDL_EventType::SDL_MOUSEBUTTONDOWN as u32 {
                        if let Some(func) = &self.on_mouse_button_down {
                            let err = func(&mut cmds, event.button.button as i32, Point(event.button.x, event.button.y));
                            if let Err(re) = err {
                                error_stack.push(re);
                            }
                        }

                        if let Some(func) = &self.on_mouse_button_down_no_commands {
                            let err = func(event.button.button as i32, Point(event.button.x, event.button.y));
                            if let Err(re) = err {
                                error_stack.push(re);
                            }
                        }
                    }

                    // Handle mouse_motion event
                    if event.type_ == SDL_EventType::SDL_MOUSEMOTION as u32 {
                        if let Some(func) = &self.on_mouse_motion {
                            let err = func(&mut cmds, Point(event.motion.x, event.motion.y));
                            if let Err(re) = err {
                                error_stack.push(re);
                            }
                        }

                        if let Some(func) = &self.on_mouse_motion_no_commands {
                            let err = func(Point(event.motion.x, event.motion.y));
                            if let Err(re) = err {
                                error_stack.push(re);
                            }
                        }
                    }
                }

                // Time::update();

                SDL_SetRenderDrawColor(self.renderer.get(), 255, 255, 255, 255);
                SDL_RenderClear(self.renderer.get());

                // Rendering goes here...
                let comps = cmds.active_components.clone();
                for active_component in comps {
                    cmds.update(active_component.clone());
                }
            
                // Call update method
                if let Some(update) = &self.update {
                    let err = update(&mut cmds);
                    if let Err(re) = err {
                        error_stack.push(re);
                    }
                }

                if let Some(update) = &self.update_no_commands {
                    let err=  update();
                    if let Err(re) = err {
                        error_stack.push(re);
                    }
                }

                if error_stack.len() > 0 {
                    for error in &error_stack {
                        error.emit();
                        panic!("[Pine] failed with {} errors.", error_stack.len());
                    }
                }

                SDL_RenderPresent(self.renderer.get());
            }

            let frame_elapsed = frame_start.elapsed().as_secs_f32();

            if frame_elapsed < self.target_frame_time {
                std::thread::sleep(
                    std::time::Duration::from_secs_f32(self.target_frame_time - frame_elapsed)
                );
            }

            Time::update();
        }

        Engine::set_active_commands(None);

        unsafe {
            SDL_DestroyRenderer(self.renderer.get());
            SDL_DestroyWindow(self.handle.get());
            SDL_Quit();
        }
    }
}