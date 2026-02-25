use crate::tests::{TestResult, Testable};
use crate::prelude::*;

/// ## Description
/// Test for [`Video2D`] component. Should render a video.
pub struct VideoTest;

impl Testable for VideoTest {
    fn run() -> TestResult {
        let mut window = Window::new_no_commands("Video Test", start, update);
        window.run();

        TestResult::SUCCESS
    }
}

fn start() -> Result<(), RuntimeException> {
    let video = make!(Video2D::new_from_media(
        "MyVideo",
        60.0,
        Assets::get::<MediaFile>("karbonat_erol").unwrap(),
        VideoSettings::HIDE_ON_FINISH
    ));

    Engine::spawn(video)?;

    Ok(())
}

fn update() -> Result<(), RuntimeException> {

    Ok(())
}