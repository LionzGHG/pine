use crate::prelude::*;

/// ## Description
/// Represents a logical rendering and physics layer.
/// 
/// Layers are primarily used for:
/// - Collision filtering
/// - Physics ground detection
/// - Logical grouping of actors
/// 
/// Actors can be assigned to layers to control interaction behavior.
/// 
/// - **Item-Type**: Engine Layer Identifier
/// 
/// ## Example
/// ```
/// actor.set_layer(Layer("My Layer"));
/// ```
/// There are also default layers, such as `GROUND` used for ground detection in physics:
/// ```
/// actor.set_layer(Layer::GROUND);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Layer(pub &'static str);

impl Layer {
    pub const DEFAULT: Layer = Layer("Default");
    pub const GROUND: Layer = Layer("Ground");
}

pub type CollisionCallback = fn(&mut Actor);

/// ## Description
/// **Axis-aligned bounding box** (AABB) collision attribute.
/// 
/// When attached to an [Actor], this attribute:
/// - Computes world-space bounds
/// - Detects intersections with other colliders
/// - Optionally triggers a **collision callback**
/// 
/// Used for:
/// - Trigger events
/// - Basic 2D collision detection
/// 
/// - **Item-Type**: Attribute (Collision)
/// 
/// ## Example
/// ```
/// actor.add_attribute(
///     Collision2D::new("Player", actor.get_size(), on_hit),
///     "collider"
/// );
/// ```
/// Or without collision callback:
/// ```
/// actor.add_attribute(
///     Collision2D::new_no_callback("Player", actor.get_size()),
///     "collider"
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Collision2D {
    pub owner: String,
    pub size: Vec2,
    pub on_collision: Option<CollisionCallback>,
}

impl Collision2D {
    pub fn new(owner: impl Into<String>, size: Vec2, callback: fn(&mut Actor)) -> Self {
        Self {
            owner: owner.into(),
            size,
            on_collision: Some(callback)
        }
    }

    pub fn new_no_callback(owner: impl Into<String>, size: Vec2) -> Self {
        Self {
            owner: owner.into(),
            size,
            on_collision: None
        }
    }

    pub fn bounds(&self) -> (Vec2, Vec2) {
        let actor_rc = Engine::find_as_ptr::<Actor>(&self.owner)
            .expect("[PINE] ACTOR NOT FOUND!");

        let actor = actor_rc.borrow();
        let t = &actor.transform;

        let half_w = self.size.x * t.scale / 2.;
        let half_h = self.size.y * t.scale / 2.;

        (
            Vec2::new(t.x - half_w, t.y - half_h),
            Vec2::new(t.x + half_w, t.y + half_h)
        )
    }

    pub fn collides_with(&self, other: &Collision2D) -> bool {
        let (a1, a2) = self.bounds();
        let (b1, b2) = other.bounds();

        a1.x < b2.x &&
        a2.x > b1.x &&
        a1.y < b2.y &&
        a2.y > b1.y 
    }
}

impl Attribute for Collision2D {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn update(&mut self) {
        let all_components = Engine::with_commands(|cmds| {
            cmds.active_components.clone()
        }).unwrap();

        for component in all_components {
            let actor_rc = match component.safecast_rc::<Actor>() {
                Some(a) => a,
                None => continue,
            };

            {
                let actor = actor_rc.borrow();
                if actor.component_id() == self.owner {
                    continue;
                }
            }

            let other_collider = {
                let actor = actor_rc.borrow();
                actor.get_first_attribute_as_ptr::<Collision2D>()
            };
            
            if let Some(other_collider) = other_collider {
                if self.collides_with(&other_collider.borrow()) {
                    if let Some(callback) = self.on_collision {
                        let actor = component.safecast_rc::<Actor>().unwrap();
                        let mut actor = actor.borrow_mut();
                        callback(&mut actor);
                    }
                }
            }
        }
    }
}

/// ## Description
/// Simple gravity-based vertical physics attribute.
/// 
/// Applies:
/// - Constant gravitational acceleration
/// - Vertical velocity integration
/// - Ground collision resolution
/// 
/// Requires:
/// - A [Collision2D] attribute on the same actor
/// - A defined ground layer for collision filtering
/// 
/// - **Item-Type**: Attribute (Physics)
/// 
/// ## Example
/// ```
/// actor.add_attribute(
///     Physics2D::new("Player", Layer::GROUND),
///     "physics"
/// );
/// ```
///
/// ## Technical Info
/// - Uses `Time::delta()`
/// - Caps delta to prevent large physics jumps
/// - Performs AABB overlap resolution vertically only
#[derive(Debug, Clone)]
pub struct Physics2D {
    pub owner: String,
    pub velocity_y: f32,
    pub gravity: f32,
    pub ground_layer: Layer,
}

impl Physics2D {
    pub fn new(owner: impl Into<String>, ground_layer: Layer) -> Self {
        Self {
            owner: owner.into(),
            velocity_y: 0.,
            gravity: 980.,
            ground_layer
        }
    }
}

impl Attribute for Physics2D {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }

    fn update(&mut self) {
        let dt = Time::delta().min(0.033);

        // Apply velocity
        self.velocity_y += self.gravity * dt;

        // Move vertically
        let actor_rc = Engine::find_as_ptr::<Actor>(&self.owner)
            .expect("[PINE] ACTOR NOT FOUND!");
        
        {
            let mut actor = actor_rc.borrow_mut();
            actor.transform.y += self.velocity_y * dt;
        }

        // Get own collider
        let collider = {
            let actor = actor_rc.borrow();
            actor.get_first_attribute_as_ptr::<Collision2D>()
                .expect("[PINE] Physics2D requires Collision2D")
        };

        let (_, self_max) = collider.borrow().bounds();

        let all_components = Engine::with_commands(|cmds| {
            cmds.active_components.clone()
        }).unwrap();

        for component in all_components {
            let other_rc = match component.safecast_rc::<Actor>() {
                Some(a) => a,
                None => continue,
            };

            {
                let other = other_rc.borrow();
                
                if other.component_id() == self.owner {
                    continue;
                }
                
                if other.layer != self.ground_layer {
                    continue;
                }
            }

            let other_collider = {
                let other = other_rc.borrow();
                other.get_first_attribute_as_ptr::<Collision2D>()
            };

            if let Some(other_collider) = other_collider {
                let collider_borrowed = collider.borrow();
                if collider_borrowed.collides_with(&other_collider.borrow()) {
                    drop(collider_borrowed);
                    
                    self.velocity_y = 0.;

                    let (other_min, _) = other_collider.borrow().bounds();

                    let mut actor = actor_rc.borrow_mut();
                    actor.transform.y -= self_max.y - other_min.y;

                    break;
                }
            }
        }
    }
}
