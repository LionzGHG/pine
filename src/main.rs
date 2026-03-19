use crate::game::begin;
use pine::tests::{card_game_test::CardGameTest, movement_test::MovementTest, poker_game::PokerGame, texture_test::TextureTest, *};

pub mod game;

fn main() {
    Test::start::<TextureTest>().verify();
}

fn begin_poker_game() {
    begin();
}