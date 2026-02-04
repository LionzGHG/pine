
use pine::prelude::*;

use crate::poker::poker_utils::{Card, Rank, Suit};

pub mod poker_utils;

pub fn begin() {
    let mut window = Window::new("Poker", start, update);
    window.on_mouse_button_down(mouse_callback);
    window.run();
}

fn mouse_callback(commands: &mut Commands, keycode: i32, pos: Point) {
    if keycode == KeyCode::LMB {
        let mut card1 = find!(commands, Actor, "Card1");
        let _rank = *card1.get_attribute::<Rank>("Card1_rank").unwrap();
        let _suit = *card1.get_attribute::<Suit>("Card1_suit").unwrap();

        if card1.texture.file_path.ends_with("King_Spades.png") {
            card1.change_texture("Card_Hidden");
        } else {
            card1.change_texture("King_Spades");
        }
    }
}

fn start(commands: &mut Commands) {
    let card = Card::new("Card1", "King_Spades", Rank::K, Suit::Spades);
    get!(card.0).transform.set_position(commands.center().to_point());
    get!(card.0).transform.scale = 0.25;
    commands.spawn(card.0.clone());
}

fn update(commands: &mut Commands) {}