

use std::cell::RefMut;

use pine::prelude::*;

use crate::game::poker_utils::{Card, Player, Rank, Suit};

pub mod poker_utils;

pub fn begin() {
    let mut window = Window::new("Poker", start, update);
    window.on_mouse_button_down(mouse_callback);
    window.run();
}

fn card_interaction(mut card1: RefMut<'_, Actor>, keycode: i32, pos: Point) {
        if keycode == KeyCode::LMB && pos.in_area(
        Point(card1.transform.x, card1.transform.y), 
        Point(card1.transform.x + card1.transform.width as i32, card1.transform.y + card1.transform.height as i32)
    ) {
        if card1.get_texture().file_path.ends_with("King_Spades.png") {
            card1.change_texture("Card_Hidden");
        } else {
            card1.change_texture("King_Spades");
        }
    }
}

fn mouse_callback(commands: &mut Commands, keycode: i32, pos: Point) {
    let card1 = find!(commands, Actor, "player1_card1");
    card_interaction(card1, keycode, pos);

    let card2 = find!(commands, Actor, "player1_card2");
    card_interaction(card2, keycode, pos);


    //if keycode == KeyCode::LMB {
    //    let mut card1 = find!(commands, Actor, "Card1");
    //    let _rank = *card1.get_attribute::<Rank>("Card1_rank").unwrap();
    //    let _suit = *card1.get_attribute::<Suit>("Card1_suit").unwrap();
    //
    //    if card1.get_texture().file_path.ends_with("King_Spades.png") {
    //        card1.change_texture("Card_Hidden");
    //    } else {
    //        card1.change_texture("King_Spades");
    //    }
    //}
}

fn start(commands: &mut Commands) {
    let player = Player::new("player1", &commands);
    commands.spawn(player.cards[0].0.clone());
    commands.spawn(player.cards[1].0.clone());

    //let card = Card::new("Card1", "King_Spades", Rank::K, Suit::Spades);
    //get!(card.0).transform.set_position(Point(commands.half_width() / 2, commands.half_height() / 2));
    //get!(card.0).transform.scale = 0.2;
    //commands.spawn(card.0.clone());
}

fn update(commands: &mut Commands) {}