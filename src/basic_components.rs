use sdl2::sys::{SDL_Rect, SDL_RenderCopy};

use crate::{Component, assets::Asset, basic_attributes::{Texture2D, Transform}};

/// ## Description
/// **Item-Type**. [Basic Component](crate::basic_components).
/// 
/// An [Actor] is a special kind of [Component] in your game that is interactable, that means it can change
/// state according to player actions in real time.
/// 
/// An Actor contains a few important fields, such as:
/// - `Transform`: The [Transform] [attribute](crate::basic_attributes) allows Actors to move inside of the game world
/// - `Texture` the [Texture2D] [attribute](crate::basic_attributes) allows us to attach a texture to the actor
pub struct Actor {
    pub id: String,
    pub transform: Transform,
    pub texture: Texture2D,
}

impl Asset for Actor {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Asset> {
        unimplemented!()
    }
}

impl Actor {
    pub fn new(id: impl Into<String>, texture: Texture2D) -> Self {
        Self {
            id: id.into(),
            transform: Transform::default(),
            texture,
        }
    }
}

impl Component for Actor {
    fn component_id(&self) -> String {
        self.id.clone()
    }
    
    fn init(&self, handle: &mut crate::Handle) {
        todo!()
    }

    fn render(&mut self, renderer: &mut crate::Renderer) {
        let dstrect = SDL_Rect {
            x: self.transform.x,
            y: self.transform.y,
            w: self.transform.width as i32,
            h: self.transform.height as i32
        };

        todo!()
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
}