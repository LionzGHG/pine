
use pine::prelude::*;

use crate::game::poker_utils::{Card, Rank, Suit};

pub mod poker_utils;

pub fn begin() {
    let mut window = Window::new(
        "Poker", 
        on_start, 
        on_tick
    );

    window.set_logical_size(10, 10);
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
        
        let (w, h) = commands.window_bounds;

        actor.transform.x = w as f32 / 2.;
        actor.transform.y = h as f32 / 2.;

        actor.transform.scale = 0.001;

        *active_card = Some(card.clone());
    }
}

fn on_start(commands: &mut Commands) {
    commands.set_logical_size(10., 10.);

    let mut deck = Vec::<Card>::new();

    for &rank in &Rank::all() {
        for &suit in &Suit::all() {
            let card = Card::new(&format!("{:?}_{:?}", rank, suit), "Card_Hidden", rank, suit);

            let (w, h) = commands.window_bounds;
            get!(card.0).transform.x = w as f32 / 2.;
            get!(card.0).transform.y = h as f32 / 2.;

            get!(card.0).transform.scale = 0.01;

            deck.push(card.clone());
            commands.spawn(card.0.clone());
        }
    }

    commands.add_global_var("deck", &deck);
    
    let active_card: Option<Card> = None;
    commands.add_global_var("active_card", &active_card);
}

fn on_tick(commands: &mut Commands) {
    let mut tmp = commands.clone();
    let mut active_card = load!(&mut tmp, Option<Card>, "active_card");

    if let Some(card) = active_card.clone() {
        let mut actor = get!(card.0);

        let (w, h) = commands.window_bounds;
        let target_x = w as f32 / 2.;
        let target_y = h as f32 / 2.0 + 2.0;

        actor.transform.x = Math::lerp(actor.transform.x, target_x, 0.1);
        actor.transform.y = Math::lerp(actor.transform.y, target_y, 0.1);

        if (actor.transform.x - target_x).abs() < 0.01 && (actor.transform.y - target_y).abs() < 0.01 {
            actor.transform.x = 4.;
            actor.transform.y = 5.;
            *active_card = None;
        }
    }
}