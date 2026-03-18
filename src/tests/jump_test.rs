
use crate::{prelude::*, tests::Testable};

pub struct JumpTest;

impl Testable for JumpTest {
    fn run() -> super::TestResult {
        let mut window = Window::new_no_commands("My Window", start, update);
        window.on_key_down_no_commands(on_key);
        window.run();

        super::TestResult::SUCCESS
    }
}

fn on_key(keycode: i32) -> Result<(), RuntimeException> {
    let player = Engine::get_actor("Player")?;
    let is_grounded = load!("is_grounded", bool);

    if keycode == KeyCode::SPACE && *is_grounded {
        *is_grounded = false;

        Engine::capture(player.clone(), |p| {
            let pos = p.transform.get_position();

            (pos.y as f64).make_global("jump_start_y")?;
            ((pos.y - 200.0) as f64).make_global("jump_target_y").unwrap();
            0.0.make_global("jump_t").unwrap();
        });
    }

    Ok(())
}

fn on_collision(other: &mut Actor) {
    *load!("is_grounded", bool) = true; 
    println!("Collided with ground!");
}

fn start() -> Result<(), RuntimeException> {

    false.make_global("is_grounded")?;

    let player = make!(Actor::new("Player", ""));
    
    Engine::capture(player.clone(), |p| {
        p.set_color(Color::GREEN);
        p.set_size(vec2![100, 100]);
        p.set_position(Engine::get_world_center());

        p.add_attribute(
            Collision2D::new("Player", p.get_size(), on_collision), 
            "Player_Collision2D"
        );

        p.add_attribute(
            Physics2D::new("Player", Layer::GROUND),
            "Player_Physics2D"
        );
    });

    let ground = make!(Actor::new("Ground", ""));

    Engine::capture(ground.clone(), |g| {
        g.set_size(vec2![Engine::get_width(), 30]);
        g.set_position(Engine::get_world_bottom());
        g.set_color(Color::RED);

        g.add_attribute(
            Collision2D::new_no_callback("Ground", g.get_size()),
            "Ground_Collision2D"
        );
    });

    Engine::spawn(player)?;
    Engine::spawn(ground)?;

    Ok(())
}

fn update() -> Result<(), RuntimeException> {

    Ok(())
}