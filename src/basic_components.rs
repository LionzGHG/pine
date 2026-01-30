use std::{cell::{RefCell, RefMut}, rc::Rc};

use sdl2::sys::{SDL_Rect, SDL_RenderCopy, SDL_Texture};

use crate::{Attribute, Component, assets::Assets, basic_attributes::{Texture2D, Transform}, util::AttributeSafecast};

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
    pub attributes: Vec<(String, Rc<RefCell<dyn Attribute>>)>,
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
            attributes: Vec::new()
        }
    }

    /// ## Description
    /// This method allows you to add a new [Attribute] to an existing [Actor]. This attribute is then added to the actor's list
    /// of active attributes. The attribute you add is assigned a unique `id`.
    /// To get an instance of an attribute within the list of active attributes in an actor, use the actor's [get_attribute](Actor::get_attribute)
    /// method and specify the unique id of said attribute.
    /// ## Example
    /// ```
    /// fn start(commands: &mut Commands) {
    ///     let player = make!(Actor::new("Player", "player"));
    ///     player.add_attribute(BoxCollider::default(), "player_collider");
    ///     commands.spawn(player.clone());   
    /// }
    /// 
    /// fn on_key_down(commands: &mut Commands) {
    ///     let player = find!(commands, Actor, "Player");
    ///     let collider = player.get_attribute::<BoxCollider>("player_collider").unwrap();
    ///     // -- snip --
    /// }
    /// ```
    pub fn add_attribute(&mut self, attr: impl Attribute + 'static + Clone, id: impl Into<String>) {
        self.attributes.push((id.into(), Rc::new(RefCell::new(attr))));
    }

    /// ## Description
    /// Use this method to retrieve a specific [Attribute] from the [actors](Actor) list of **active attributes**.
    /// 
    /// Use [add_attribute](Actor::add_attribute) to **add**  attributes to an actor.
    /// ## Example
    ///     /// ## Example
    /// ```
    /// fn start(commands: &mut Commands) {
    ///     let player = make!(Actor::new("Player", "player"));
    ///     player.add_attribute(BoxCollider::default(), "player_collider");
    ///     commands.spawn(player.clone());   
    /// }
    /// 
    /// fn on_key_down(commands: &mut Commands) {
    ///     let player = find!(commands, Actor, "Player");
    ///     let collider = player.get_attribute::<BoxCollider>("player_collider").unwrap();
    ///     // -- snip --
    /// }
    /// ```
    pub fn get_attribute<T: Attribute + 'static>(&self, id: &str) -> Option<RefMut<'_, T>> {
        let index = self.attributes
            .iter()
            .position(|x| x.0 == id)?;

        self.attributes[index].1.safecast_ref::<T>()
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