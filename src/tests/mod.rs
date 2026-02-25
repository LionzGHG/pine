
pub mod video_test;

/// ## Description
/// see: [Test]
pub trait Testable {
    /// ## Description
    /// see: [Test]
    fn run() -> TestResult;
}

/// ## Description
/// see: [Test]
pub enum TestResult {
    SUCCESS,
    FAILURE(String)
}

impl TestResult {
    /// ## Description
    /// see: [Test]
    pub fn verify(&self) {
        match self {
            Self::SUCCESS => {
                println!("[PINE] Test finished successfully!");
            },
            Self::FAILURE(reason) => {
                panic!("[PINE] Test failed: {}.", reason);
            }
        }
    }
}

/// ## Description
/// `Test` is used to run tests.
/// 
/// The [`start`](Test::start) method of this struct allows you to start any [testable](Testable) struct:
/// 
/// ```
/// let result: TestResult<'_> = Test::run::<TestableStruct>();
/// ```
/// A testable struct has to implement the [`Testable`] trait:
/// ```
/// pub struct TestableStruct;
/// 
/// impl Testable for TestableStruct {
///     fn run() -> TestResult<'_> {
///         if 10 == 10 {
///             TestResult::SUCCESS
///         } else {
///             TestResult::FAILURE("???")
///         }
///     }
/// }
/// ```
/// Use [`.verify()`](TestResult::verify) on [`TestResult<'_>`] to handle the result:
/// ```
/// fn main() {
///     // this runs the test for the `Video2D` component:
///     pine::tests::Test::run::<VideoTest>().verify();
/// }
/// ```
pub struct Test;

impl Test {
    /// ## Description
    /// see: [Test]
    pub fn start<T: Testable>() -> TestResult {
        T::run()
    }
}

use crate::prelude::*;

/// ## Description
/// Minimal test implementation.
pub struct MinimalTest;

impl Testable for MinimalTest {
    fn run() -> TestResult {
        let mut window = Window::new_no_commands("Minimal Example", minimal_start, minimal_update);
        window.set_logical_size(800, 600);
        window.on_key_down_no_commands(key_callback);
        window.run();

        TestResult::SUCCESS
    }
}

fn key_callback(keycode: i32) -> Result<(), RuntimeException> {
    let player = Engine::get_actor("Player")?;

    Engine::capture(player, |p| {
        match keycode {
            KeyCode::UP     => p.transform.y -= 10.,
            KeyCode::DOWN   => p.transform.y += 10.,
            KeyCode::LEFT   => p.transform.x -= 10.,
            KeyCode::RIGHT  => p.transform.x += 10.,
            _               => (),
        }
    });

    Ok(())
}

fn minimal_start() -> Result<(), RuntimeException> {
    let player = make!(Actor::new("Player", ""));

    Engine::capture(player.clone(), |player| {
        player.set_size(vec2![100, 100]);
        player.set_position(Engine::get_world_center());

        player.set_color(Color::RED);
    });

    Engine::spawn(player)?;

    Ok(())
}

fn minimal_update() -> Result<(), RuntimeException> {
    Ok(())
}