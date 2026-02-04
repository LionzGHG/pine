
use sdl2::sys::{SDL_QueryTexture, SDL_Texture};

pub use crate::Attribute;
use crate::{assets::{Asset, AssetExt}, math::Point};

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
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize,
    pub scale: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            scale: 1.,
        }
    }
}

impl Transform {
    pub fn as_point(&self) -> Point {
        Point(self.x, self.y)
    }

    pub fn set_position(&mut self, p: Point) {
        self.x = p.x();
        self.y = p.y();
    }
}

/// ## Description
/// **Item-Type**: [Basic Attribute](crate::basic_attributes).
/// 
/// [Textures](Texture2D) are used to upload images onto the screen. By using the [Texture2D] [attribute](crate::Attribute)
/// you can link an image file to a specific [Component](crate::Component) in your game world.
/// 
/// Use the [Asset browser](crate::assets::Assets) to upload an image file using the [new](Texture2D::new) function.
/// ## Example
/// ```
/// fn __start__(commands: &mut Commands) {
///     let player_image = Assets::get<Texture2D>("player.png").unwrap();
///     let texture = Texture2D::new(player_image);
///     let player_actor = Actor::new("Player", texture);
/// }
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
