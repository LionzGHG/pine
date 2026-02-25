
use pine::prelude::*;

pub mod poker_utils;

pub fn begin() {
    let mut window = Window::new_no_commands("My Window", start, update);
    window.set_logical_size(800, 600);
    window.on_key_down_no_commands(on_key);
    window.run();
}     

fn on_key(keycode: i32) -> Result<(), RuntimeException> {
    if keycode == KeyCode::SPACE {
        Engine::capture(Engine::get_actor("Actor1")?, |actor| {
            println!("height={}, y={}", Engine::get_height(), actor.transform.y);
            actor.transform.y -= 200.0;
            println!("height={}, y={}", Engine::get_height(), actor.transform.y);            
        });
    }

    Ok(())
}

fn on_collision(other: &mut Actor) {
    println!("Actor1 collided with Actor2.");
}

fn start() -> Result<(), RuntimeException> {    
    let actor1 = make!(Actor::new("Actor1", ""));

    Engine::capture(actor1.clone(), |actor| {
        actor.set_color(Color::GREEN);
        actor.set_size(vec2![100, 100]);

        let world_center = Engine::get_world_center();
        actor.set_position(world_center - vec2![0, 0]);

        actor.add_attribute(
            Collision2D::new("Actor1", actor.get_size(), on_collision),
            "Actor1_Collider2D"
        );

        actor.add_attribute(
            Physics2D::new("Actor1", Layer::GROUND),
            "Actor1_Physics2D"
        );
    });

    Engine::spawn(actor1)?;

    let actor2 = make!(Actor::new("Actor2", ""));

    Engine::capture(actor2.clone(), |actor| {
        actor.set_color(Color::BLUE);
        actor.set_size(vec2![100, 100]);
        actor.set_position(Engine::get_world_center());
        actor.set_layer(Layer::GROUND);

        actor.add_attribute(
            Collision2D::new_no_callback("Actor2", actor.get_size()),
            "Actor2_Collider2D"
        );
    });

    Engine::spawn(actor2)?;

    Ok(())
}

fn update() -> Result<(), RuntimeException> {

    Ok(())
}

