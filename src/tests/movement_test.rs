use crate::tests::Testable;
use crate::prelude::*;

pub struct MovementTest;

impl Testable for MovementTest {
    fn run() -> super::TestResult {
        let mut window = Window::new_no_commands("Movement Test", start, update);
        window.set_logical_size(800, 600);
        window.on_key_down_no_commands(key_down);

        window.run();
        super::TestResult::SUCCESS
    }
}

fn key_down(keycode: i32) -> PineResult {
    let mut collision = load!("collision", bool);
    if *collision && keycode == KeyCode::SPACE {
        *load!("jumping", bool) = true;
        *collision = false;        
    }

    let actor = Engine::get_actor("Actor1")?;
    Engine::capture(actor, |a| {
        match keycode {
            KeyCode::LEFT   => a.transform.x -= 10.,
            KeyCode::RIGHT  => a.transform.x += 10.,
            _               => (), 
        }
    });

    Ok(())
}

fn on_collision(other: &mut Actor) {
    *load!("collision", bool) = true;
}

fn start() -> PineResult {
    let actor1 = make!(Actor::new("Actor1", "player"));

    false.make_global("collision")?;
    false.make_global("jumping")?;

    Engine::capture(actor1.clone(), |a| {
        a.set_size(vec2![100, 100]);
        a.set_position(Engine::get_world_center() - vec2![0, 100]);
        a.set_color(Color::GREEN);

        a.add_attribute(
            Collision2D::new("Actor1", a.get_size(), on_collision),
            "Actor1_Collision2D"
        );

        a.add_attribute(
            Physics2D::new("Actor1", Layer::GROUND),
            "Actor1_Physics2D"
        );
    });

    Engine::spawn(actor1)?;

    let actor2 = make!(Actor::new("Actor2", ""));

    Engine::capture(actor2.clone(), |a| {
        a.set_size(vec2![1000, 100]);
        a.set_position(Engine::get_world_center() + vec2![0, 100]);
        a.set_color(Color::BLUE);
        a.set_layer(Layer::GROUND);

        a.add_attribute(
            Collision2D::new_no_callback("Actor2", a.get_size()),
            "Actor2_Collision2D"
        );
    });

    Engine::spawn(actor2)?;

    Ok(())
}

fn update() -> PineResult {
    let actor = Engine::get_actor("Actor1")?;

    let mut jumping = load!("jumping", bool);

    Engine::capture(actor, |a| {
        let current_pos = a.transform.get_position();

        if *jumping {
            let target_pos = vec2![a.transform.x, Engine::get_world_center().y - 100.];
            let new_pos = Math::lerp_vec(current_pos, target_pos, 0.1);
            a.set_position(new_pos);

            if (new_pos - target_pos).length() < 50.0 {
                *jumping = false;
            }
        }
    });

    Ok(())
}