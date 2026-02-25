
use sdl2::{pixels::Color as Sdl2Color, sys::{SDL_QueryTexture, SDL_Texture}};
use crate::util::Color;

pub use crate::Attribute;
use crate::{assets::{Asset, AssetExt}, math::{Point, Vec2}, util::Floatify};
 
/// ## Description
/// Defines the visual surface properties of a renderable element.
/// 
/// Currently, a `Material` only contains a flat [Color], but it acts as a
/// future-proof abstraction layer for more advanced rendering features such as:
/// - Shaders
/// - Lighting parameters
/// - Texture blending
/// 
/// In the engine, materials are typically associated with rendering
/// components to control how an object appears on screen.
/// 
/// An [Actor](crate::prelude::Actor) uses an [Material] whenever you create it without a [texture](Texture2D).
/// 
/// - **Item-Type**: Rendering Attribute
/// 
/// ## Example
/// ```
/// fn start() -> Result<(), RuntimeException> {
///     let actor = make!(Actor::new("My Actor", "")); // leave second parameter as empty string -> no texture
///     
///     Engine::capture(actor.clone(), |actor|) {
///         actor.set_size(100, 100); // since no texture is set, you must assign size manually
///                                   // otherwise `actor.transform.x` and `actor.transform.y` will equal 0
///         actor.set_material(Material::new(Color::RED)); // or, just use: `actor.set_color(Color::RED);`
///     }
/// 
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
}

impl Material {
    #[inline(always)]
    pub fn new(color: Color) -> Self {
        Material { color }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::BLACK
        }
    }
}

/// ## Description
/// **Item-Type**: [Basic Attribute](crate::basic_attributes).
/// 
/// The [Transform] [attribute](Attribute) allows you to modify the position `x` and `y`,
/// as well as the size `width` and `height` in the game world in real time, for example following a
/// key press.
/// 
/// By default, you should use the [Actor](crate::basic_components::Actor) [Component](crate::Component) if you
/// want an dynamic element in your game world, which derives the [Transform] attribute by default. 
/// ## Example
/// ```
/// fn __key_callback__(commands: &mut Commands, keycode: i32) {
///     if keycode == KeyCode::UP {
///         let mut player = find!(commands, Actor, "Player");
///         player.transform.position.x += 10; // player is an Actor
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub scale: f32,
    pub rotation: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            width: 0,
            height: 0,
            scale: 1.,
            rotation: 0.,
        }
    }
}

impl Transform {
    pub fn as_point(&self) -> Point {
        Point(self.x as i32, self.y as i32)
    }

    pub fn set_position(&mut self, v: Vec2) {
        self.x = v.x;
        self.y = v.y;
    }

    pub fn set_size(&mut self, s: Vec2) {
        self.width = s.x as i32;
        self.height = s.y as i32;
    }

    pub fn get_position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

/// ## Description
/// **Item-Type**: Basic Attribute / Asset.
/// 
/// Represents a 2D texture loaded from disk and uploaded to the GPU.
/// 
/// `Texture2D` serves two purposes:
/// - As an [Asset], representing a file path stored in the asset system
/// - As an [Attribute], attaching the texture to a renderable component
/// 
/// The engine resolves the file path into an SDL texture internally.
/// The `sdl_texture` pointer is managed by the renderer backend.
/// 
/// - Used for sprites, UI images, backgrounds
/// - Attached to an [Actor](crate::prelude::Actor) or rendering component
/// 
/// Usually, textures are applied to actors on creation. If you want to apply a texture to a non-actor [Component](crate::prelude::Component),
/// you'll have to add it as an [Attribute] directly.
/// ## Example
/// Textures on actors:
/// ```
/// let actor = make!(Actor::new("ActorName", "ActorTextureName"));
/// ```
/// Textures on other components:
/// ```
/// let component = make!(MyComponent::new("ComponentName"));
/// 
/// Engine::capture_any::<MyComponent>(my_component.clone(), |mc| {
///     mc.add_attribute(
///         Texture2D::new("ComponentTexturePath"),
///         "MyComponent_Texture"
///     );
/// });
/// ```
#[derive(Clone, Debug)]
pub struct Texture2D {
    pub file_path: String,
    pub sdl_texture: Option<*mut SDL_Texture>,
    pub width: i32,
    pub height: i32,
}

impl Texture2D {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
            sdl_texture: None,
            width: 0,
            height: 0,
        }
    }

    pub unsafe fn query_texture_size(tex: *mut SDL_Texture) -> (i32, i32) {
        let mut w = 0i32;
        let mut h = 0i32;

        if SDL_QueryTexture(tex, std::ptr::null_mut(), std::ptr::null_mut(), &mut w, &mut h) != 0 {
            panic!("SDL_QueryTexture failed to query texture: '{:?}'", tex);
        }

        (w, h)
    }

    pub fn get(&self) -> &str {
        &self.file_path
    }
}

impl Asset for Texture2D {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Asset> {
        Box::new((*self).clone())
    }
}

impl AssetExt for Texture2D {}

impl Attribute for Transform {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Attribute for Texture2D {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}