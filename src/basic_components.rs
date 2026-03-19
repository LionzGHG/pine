use std::{any::{type_name, type_name_of_val}, cell::{Ref, RefCell, RefMut}, ptr::null, rc::Rc};

use sdl2::{pixels::Color as Sdl2Color, sys::{SDL_BlendMode, SDL_Rect, SDL_RenderCopy, SDL_RenderCopyEx, SDL_RenderFillRect, SDL_RendererFlip, SDL_SetRenderDrawBlendMode, SDL_SetRenderDrawColor, SDL_Texture}};
use crate::{prelude::Collision2D, util::{AttributeSafecastRc, Color, ComponentSafecastRc, Downcastable}};

use crate::{Attribute, Component, assets::Assets, basic_attributes::{Material, Texture2D, Transform}, math::Vec2, prelude::Layer, util::AttributeSafecast};

/// ## Description
/// **Item-Type**. [Basic Component](crate::basic_components).
/// 
/// An [Actor] is a special kind of [Component] in your game that is interactable, that means it can change
/// state according to player actions in real time.
/// 
/// An Actor contains a few important fields, such as:
/// - `Transform`: The [Transform] [attribute](crate::basic_attributes) allows Actors to move inside of the game world
/// - `Texture` the [Texture2D] [attribute](crate::basic_attributes) allows us to attach a texture to the actor. 
/// Leave this field empty (as string: `""`) if you want an actor without a texture. Then, use the [`set_material`](Actor::set_material) method
/// to specify a material instead (otherwise, `Actor` will use the **default material**)
#[derive(Clone)]
pub struct Actor {
    pub id: String,
    pub transform: Transform,
    pub texture: Option<Texture2D>,
    pub material: Material,
    pub attributes: Vec<(String, Rc<RefCell<dyn Attribute>>)>,
    pub layer: Layer,
}

impl Actor {
    /// ## Description
    /// Create a new [Actor] from an unique ID and a [texture](Texture2D). The [Actor] [Component] will load the [texture](Texture2D)
    /// from the [asset browser](Assets) automatically, you'll just need to pass the ***name of the texture***!
    pub fn new(id: impl Into<String>, texture_name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            transform: Transform::default(),
            texture: Assets::get::<Texture2D>(&texture_name.into()),
            material: Material::default(),
            attributes: Vec::new(),
            layer: Layer::DEFAULT,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.id
    }

    pub fn set_size(&mut self, s: Vec2) {
        self.transform.set_size(s);
    }

    pub fn set_position(&mut self, p: Vec2) {
        self.transform.set_position(p);
    }

    pub fn get_size(&self) -> Vec2 {
        Vec2::new(self.transform.width, self.transform.height)
    }

    pub fn set_layer(&mut self, layer: Layer) {
        self.layer = layer;
    }

    pub fn get_layer(&self) -> Layer {
        self.layer
    }

    pub fn get_texture(&self) -> Option<&Texture2D> {
        self.texture.as_ref()
    }

    pub fn set_material(&mut self, mat: Material) {
        self.material = mat;
    }

    pub fn set_color(&mut self, color: Color) {
        self.material.color = color;
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
    pub fn get_attribute<T: Attribute + 'static>(&self, id: &str) -> Option<Ref<'_, T>> {
        let index = self.attributes
            .iter()
            .position(|x| x.0 == id)?;

        self.attributes[index].1.safecast_ref::<T>()
    }

    pub fn get_first_attribute<T: Attribute + 'static>(&self) -> Option<Ref<'_, T>> {
        let pos = self.attributes
            .iter()
            .position(|x| {
                x.1.can_be_downcast_to::<Collision2D>()
            })?;
        
        self.attributes[pos].1.safecast_ref::<T>()
    }

    pub fn get_first_attribute_as_ptr<T: Attribute + 'static>(
        &self
    ) -> Option<Rc<RefCell<T>>> {
        let component = self.attributes
            .iter()
            .find(|(_, attr)| {
                attr.can_be_downcast_to::<T>()
            })?;

        component.1.safecast_rc::<T>()
    }

    pub fn get_attribute_mut<T: Attribute + 'static>(&self, id: &str) -> Option<RefMut<'_, T>> {
        let index = self.attributes
            .iter()
            .position(|x| x.0 == id)?;

        self.attributes[index].1.safecast_ref_mut::<T>()        
    }

    pub fn change_texture(&mut self, path_to_new_texture: impl Into<String>) {
        let new_texture = Assets::get::<Texture2D>(path_to_new_texture.into().as_str()).unwrap();

        self.transform.width = 0;
        self.transform.height = 0;

        self.texture = Some(new_texture);
    }
}

impl Component for Actor {
    fn component_id(&self) -> String {
        self.id.clone()
    }

    fn get_attributes(&self) -> Vec<(String, Rc<RefCell<dyn Attribute + 'static>>)> {
        self.attributes.clone()
    }
    
    fn init(&self, handle: &mut crate::Handle) {}

    fn render(&mut self, renderer: &mut crate::Renderer) {
        if let Some(ref mut texture) = &mut self.texture {
            unsafe {
                let tex = renderer.get_or_create_texture(texture);
            
                let (w, h) = Texture2D::query_texture_size(tex);
                let scale = renderer.world_scale;

                let world_w = w as f32 * self.transform.scale;
                let world_h = h as f32 * self.transform.scale;

                let pixel_w = world_w * scale;
                let pixel_h = world_h * scale;

                let pixel_x = self.transform.x * scale - pixel_w / 2.0;
                let pixel_y = self.transform.y * scale - pixel_h / 2.0;

                let dstrect = SDL_Rect {
                    x: pixel_x as i32,
                    y: pixel_y as i32,
                    w: pixel_w as i32,
                    h: pixel_h as i32
                };

                // println!(
                //     "scale={}, tex_w={}, world_w={}, pixel_w={}",
                //     renderer.world_scale,
                //     w,
                //     world_w,
                //     pixel_w
                // );

                //SDL_RenderCopy(renderer.get(), tex, null(), &dstrect);
                SDL_RenderCopyEx(
                    renderer.get(),
                    tex,
                    std::ptr::null(),
                    &dstrect,
                    self.transform.rotation as f64,
                    std::ptr::null(),
                    SDL_RendererFlip::SDL_FLIP_NONE,
                );
            }
        } else {
            unsafe {
                let color: Sdl2Color = self.material.color.into();
            
                SDL_SetRenderDrawColor(renderer.get(), color.r, color.g, color.b, color.a);
                SDL_SetRenderDrawBlendMode(renderer.get(), SDL_BlendMode::SDL_BLENDMODE_BLEND);
            
                let scale = renderer.world_scale;
            
                // Use actor size instead of transform width/height
                let world_w = self.get_size().x * self.transform.scale;
                let world_h = self.get_size().y * self.transform.scale;
            
                let pixel_w = world_w * scale;
                let pixel_h = world_h * scale;
            
                let pixel_x = self.transform.x * scale - pixel_w / 2.0;
                let pixel_y = self.transform.y * scale - pixel_h / 2.0;
            
                // println!("pixel_w={}, pixel_h={}", pixel_w, pixel_h);
                // println!("size={:?}, transform.scale={:?}", self.get_size(), self.transform.scale);

                let rect = SDL_Rect {
                    x: pixel_x as i32,
                    y: pixel_y as i32,
                    w: pixel_w as i32,
                    h: pixel_h as i32,
                };
            
                SDL_RenderFillRect(renderer.get(), &rect);
            }
        }
    }

    fn clone_box(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }

    fn component_type(&self) -> String {
        String::from("Actor")
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

    fn get_attributes(&self) -> Vec<(String, Rc<RefCell<dyn Attribute>>)> {
        Vec::new()
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