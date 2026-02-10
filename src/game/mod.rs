
use pine::prelude::*;

use crate::game::poker_utils::{Card, Rank, Suit};

pub mod poker_utils;

pub fn begin() {
    let mut window = Window::new(
        "Poker", 
        on_start, 
        on_tick
    );
    window.on_mouse_button_down(on_mouse_down);

    window.run();
}

fn on_mouse_down(commands: &mut Commands, _keycode: i32, _p: Point) {
    let mut tmp = commands.clone();
    let mut tmp2 = commands.clone();

    let mut deck = load!(&mut tmp, Vec<Card>, "deck");
    let mut active_card = load!(&mut tmp2, Option<Card>, "active_card");

    if let Some(card) = deck.pop() {
        let mut actor = get!(card.0);
        actor.transform.x = 400;
        actor.transform.y = 300;

        *active_card = Some(card.clone());
    }
}

fn on_start(commands: &mut Commands) {
    let mut deck = Vec::<Card>::new();

    for &rank in &Rank::all() {
        for &suit in &Suit::all() {
            let card = Card::new(&format!("{:?}_{:?}", rank, suit), "Card_Hidden", rank, suit);

            get!(card.0).transform.x = 4;
            get!(card.0).transform.y = 4;

            deck.push(card.clone());
            commands.spawn(card.0.clone());
        }
    }

    commands.add_global_var("deck", &deck);
    
    let active_card: Option<Card> = None;
    commands.add_global_var("active_card", &active_card);
}

fn on_tick(commands: &mut Commands) {
    let mut active_card = load!(commands, Option<Card>, "active_card");

    if let Some(card) = active_card.clone() {
        let mut actor = get!(card.0);

        actor.transform.x = Math::lerp(actor.transform.x as f32, 4.0, 0.1) as i32;
        actor.transform.y = Math::lerp(actor.transform.y as f32, 5.5, 0.1) as i32;

        if (actor.transform.x as f32 - 4.0).abs() < 1.0 && (actor.transform.y as f32 - 5.5).abs() < 1.0 {
            actor.transform.x = 4;
            actor.transform.y = 5;
            *active_card = None;
        }
    }
}