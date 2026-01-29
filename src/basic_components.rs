use sdl2::sys::{SDL_Rect, SDL_RenderCopy, SDL_Texture};

use crate::{Component, assets::Assets, basic_attributes::{Texture2D, Transform}};

/// ## Description
/// **Item-Type**. [Basic Component](crate::basic_components).
/// 
/// An [Actor] is a special kind of [Component] in your game that is interactable, that means it can change
/// state according to player actions in real time.
/// 
/// An Actor contains a few important fields, such as:
/// - `Transform`: The [Transform] [attribute](crate::basic_attributes) allows Actors to move inside of the game world
/// - `Texture` the [Texture2D] [attribute](crate::basic_attributes) allows us to attach a texture to the actor
#[derive(Clone)]
pub struct Actor {
    pub id: String,
    pub transform: Transform,
    pub texture: Texture2D,
}

impl Actor {
    /// ## Description
    /// Create a new [Actor] from an unique ID and a [texture](Texture2D). The [Actor] [Component] will load the [texture](Texture2D)
    /// from the [asset browser](Assets) automatically, you'll just need to pass the ***name of the texture***!
    pub fn new(id: impl Into<String>, texture_name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            transform: Transform::default(),
            texture: Assets::get::<Texture2D>(&texture_name.into()).expect("ACTOR: Failed to load texture!"),
        }
    }
}

impl Component for Actor {
    fn component_id(&self) -> String {
        self.id.clone()
    }
    
    fn init(&self, handle: &mut crate::Handle) {}

    fn render(&mut self, renderer: &mut crate::Renderer) {
        let dstrect = SDL_Rect {
            x: self.transform.x,
            y: self.transform.y,
            w: self.transform.width as i32,
            h: self.transform.height as i32
        };

        unsafe {
            let sdl2_texture: *mut SDL_Texture = renderer.texture_from_asset(&self.texture);
    
            SDL_RenderCopy(
                renderer.get(),
                sdl2_texture,
                std::ptr::null(),
                &dstrect as *const SDL_Rect
            );
        }
    }

    fn clone_box(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// ## Description
/// **Item-Type**: [Basic Component](crate::basic_components).
/// 
/// A [Label] is a **basic UI component** that allows you to render 2D text onto the screen. Use `Label::new` to
/// create a new basic label. Pass arguments for `id`, `text` as well as `x` and `y` position. 
/// ## Example
/// ```
/// let label = Label::new("Label1", "Hello, World!", 100, 100);
/// ```
#[derive(Clone)]
pub struct Label {
    pub id: String,
    pub text: String,
    pub x: i32,
    pub y: i32,
}

impl Label {
    pub fn new(id: impl Into<String>, text: impl Into<String>, x: i32, y: i32) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            x,
            y
        }
    }
}

impl Component for Label {
    fn init(&self, handle: &mut crate::Handle) {
        // TODO!
    }

    fn render(&mut self, renderer: &mut crate::Renderer) {
        // TODO!
    }

    fn component_id(&self) -> String {
        self.id.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
}