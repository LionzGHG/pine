
use pine::prelude::*;

use crate::game::poker_utils::{Card, Rank, Suit};

pub mod poker_utils;

pub fn begin() {
    let mut window = Window::new_no_commands("My Window", start, update);
    window.on_mouse_button_down_no_commands(on_mouse);
    window.run();
}

fn on_mouse(_key: i32, _p: Point) -> Result<(), RuntimeException> {
    let mut deck = Engine::get_global_var::<Vec<Card>>("deck");
    let mut active_card = Engine::get_global_var::<Option<Card>>("active_card");

    if active_card.is_none() {
        if let Some(card) = deck.pop() {
            *active_card = Some(card.clone());
        }
    }

    Ok(())
}

fn start() -> Result<(), RuntimeException> {
    let mut deck = Vec::<Card>::new();

    for &rank in &Rank::all() {
        for &suit in &Suit::all() {
            let card = Card::new(
                &format!("{rank:?}_{suit:?}"),
                "Card_Hidden",
                rank,
                suit
            );

            let (w, h) = Engine::get_world_size();
            get!(card.0).transform.x = w / 2.;
            get!(card.0).transform.y = h / 2.;

            get!(card.0).transform.scale = 0.1;

            deck.push(card.clone());
            Engine::spawn(card.0)?;
        }
    }

    Engine::add_global_var("deck", &deck)?;

    let active_card: Option<Card> = None;
    Engine::add_global_var("active_card", &active_card)?;

    Ok(())
}

fn update() -> Result<(), RuntimeException> {
    let mut active_card = Engine::get_global_var::<Option<Card>>("active_card");

    if let Some(card) = active_card.clone() {
        let mut actor = get!(card.0);

        let (w, h) = Engine::get_world_size();
        let target_x = w / 2.;
        let target_y = h / 2. + 100.;

        actor.transform.x = Math::lerp(actor.transform.x, target_x, 0.01);
        actor.transform.y = Math::lerp(actor.transform.y, target_y, 0.01);

        if (actor.transform.x - target_x).abs() < 0.1 && (actor.transform.y - target_y).abs() < 0.1 {
            *active_card = None;
        }
    }

    Ok(())
}
