use crate::game::begin;
use pine::tests::{video_test::VideoTest, *};

pub mod game;

fn main() {
    Test::start::<VideoTest>().verify();
}

fn begin_poker_game() {
    begin();
}