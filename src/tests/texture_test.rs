
use crate::{prelude::*, tests::Testable};

pub struct TextureTest;

impl Testable for TextureTest {
    fn run() -> super::TestResult {
        let mut window = Window::new_no_commands("Texture test", start, update);

        window.run();
        super::TestResult::SUCCESS
    }
}

fn start() -> PineResult {
    let player = make!(Actor::new("Player", "player"));

    Engine::capture(player.clone(), |p| {
        p.set_position(Engine::get_world_center());
    });

    Engine::spawn(player)?;

    Ok(())
}

fn update() -> PineResult {

    Ok(())
}