use crate::tests::{TestResult, Testable};
use crate::prelude::*;

pub struct CardGameTest;

impl Testable for CardGameTest {
    fn run() -> super::TestResult {
        let mut window = Window::new_no_commands(
            "Card Game Test",
            start,
            update
        );

        window.set_logical_size(1000, 600);
        window.on_mouse_button_down_no_commands(on_mouse);
        window.run();

        TestResult::SUCCESS
    }
}

fn on_mouse(keycode: i32, pos: Point) -> Result<(), RuntimeException> {
    // increase score on click
    let mut score = Engine::get_global_var::<i32>("score");
    *score += 1;

    Ok(())
}

fn start() -> Result<(), RuntimeException> {
    // global score
    let score = 0i32;
    Engine::add_global_var("score", &score)?;

    // create deck (simplified: only 3 cards)
    spawn_card("Card1", "Card_Hidden", vec2![200, 300])?;
    spawn_card("Card2", "Card_Hidden", vec2![400, 300])?;
    spawn_card("Card3", "Card_Hidden", vec2![600, 300])?;

    Ok(())
}

fn update() -> Result<(), RuntimeException> {

    Ok(())
}

fn spawn_card(id: &str, texture: &str, pos: Vec2) -> Result<(), RuntimeException> {
    let card = make!(Actor::new(id, texture));

    Engine::capture(card.clone(), |c| {
        c.set_size(vec2![120, 180]);
        c.set_position(pos);
    });

    Engine::spawn(card)?;

    Ok(())
}