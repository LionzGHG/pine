
use pine::prelude::*;

use crate::game::poker_utils::{Card, Rank, Suit};

pub mod poker_utils;

pub fn begin() {
    let mut window = Window::new_no_commands("My Window", start, update);
    window.set_logical_size(10, 10);
    window.on_mouse_button_down_no_commands(on_mouse);
    window.run();
}

fn on_mouse(key: i32, _p: Point) {
    let mut deck = Engine::with_commands_static(|commands: &'static mut Commands| {
        commands.get_global_var::<Vec<Card>>("deck").unwrap()
    }).unwrap();

    let mut active_card = Engine::with_commands_static(|commands| {
        commands.get_global_var::<Option<Card>>("active_card").unwrap()
    }).unwrap();

    if let Some(card) = deck.pop() {
        *active_card = Some(card.clone());
    }
}

fn start() {
    let mut deck = Vec::<Card>::new();

    for &rank in &Rank::all() {
        for &suit in &Suit::all() {
            let card = Card::new(
                &format!("{rank:?}_{suit:?}"),
                "Card_Hidden",
                rank,
                suit
            );

            let (w, h) = Engine::with_commands(|commands| {
                commands.window_bounds
            }).unwrap();

            get!(card.0).transform.x = w as f32 / 2.;
            get!(card.0).transform.y = h as f32 / 2.;

            get!(card.0).transform.scale = 0.1;

            deck.push(card.clone());
            Engine::with_commands(move |commands| {
                commands.spawn(card.0.clone());
            });
        }
    }

    Engine::with_commands(move |commands| {
        commands.add_global_var("deck", &deck);
    });

    let active_card: Option<Card> = None;
    Engine::with_commands(move |commands| {
        commands.add_global_var("active_card", &active_card);
    });
}

fn update() {
    let mut active_card = Engine::with_commands_static(|cmds| {
        cmds.get_global_var::<Option<Card>>("active_card").unwrap()
    }).unwrap();

    if let Some(card) = active_card.clone() {
        let mut actor = get!(card.0);

        let (w, h) = Engine::with_commands(|cmds| cmds.window_bounds).unwrap();
        let target_x = w as f32 / 2.;
        let target_y = h as f32 / 2. + 100.;

        actor.transform.x = Math::lerp(actor.transform.x, target_x, 0.001);
        actor.transform.y = Math::lerp(actor.transform.y, target_y, 0.001);

        if (actor.transform.x - target_x).abs() < 0.01 && (actor.transform.y - target_y).abs() < 0.01 {
            *active_card = None;
        }
    }
}
