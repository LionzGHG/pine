use std::cmp::Ordering;
use std::sync::Once;

use crate::tests::{TestResult, Testable};
use crate::prelude::*;
use crate::tests::poker_game_utils::*;

pub struct PokerGame;

impl Testable for PokerGame {
    fn run() -> super::TestResult {
        let mut window = Window::new_no_commands(
            "Blackjack Game",
            start,
            update
        );
        window.set_logical_size(800, 600);
        window.on_key_down_no_commands(on_click);
        window.run();

        TestResult::SUCCESS
    }
}

fn on_click(keycode: i32) -> Result<(), RuntimeException> {

    if keycode == KeyCode::KEY_H {
        // println!("You've hit!");
        let mut hit = load!("hit_signal", bool);
        *hit = true;
    }

    if keycode == KeyCode::KEY_D {
        // println!("You've doubled down!");
        let mut dd = load!("double_down_signal", bool);
        *dd = true;
    }

    if keycode == KeyCode::KEY_S {
        // println!("You stand!");
        let mut stand = load!("stand_signal", bool);
        *stand = true;
    }

    Ok(())
}

static INIT_SIGNALS: Once = Once::new();

fn ensure_signals_initialized() -> Result<(), RuntimeException> {
    let mut result = Ok(());

    INIT_SIGNALS.call_once(|| {
        result = init_signals();
    });

    result
}

fn reset_signals() {
    *load!("hit_signal", bool) = false;
    *load!("double_down_signal", bool) = false;
    *load!("stand_signal", bool) = false;
    *load!("bust_signal", bool) = false;
    *load!("dealer_bust_signal", bool) = false;

    *load!("round_over", bool) = false;
    *load!("lock_hit", bool) = false;

    *load!("drawn_cards", i32) = 0;
    *load!("player_card_value", i32) = 0;
    *load!("dealer_card_value", i32) = 0;
}

fn init_signals() -> Result<(), RuntimeException> {

    // signals
    false.make_global("hit_signal")?;
    false.make_global("double_down_signal")?;
    false.make_global("stand_signal")?;
    false.make_global("bust_signal")?;
    false.make_global("dealer_bust_signal")?;

    // round state
    false.make_global("round_over")?;
    false.make_global("lock_hit")?;

    // counters
    0i32.make_global("drawn_cards")?;
    0i32.make_global("dealer_wins")?;
    0i32.make_global("player_wins")?;

    // card values
    0i32.make_global("player_card_value")?;
    0i32.make_global("dealer_card_value")?;

    // deck placeholder
    let deck = Deck::new();
    upload!(&deck => "deck")?;

    Ok(())
}

fn start() -> Result<(), RuntimeException> {

    // Ensure that all global variables are initialized:
    // only runs ONCE at the start of the program
    ensure_signals_initialized()?;

    // reset necessary global variables, whenever a new
    // round i started:
    reset_signals();

    // new deck each round
    let mut deck = Deck::new();
    upload!(&deck => "deck")?;

    let mut player_card_value = load!("player_card_value", i32);
    let mut dealer_card_value = load!("dealer_card_value", i32);

    // Generate Player Cards (Hidden)
    let card1 = deck.draw_card();

    Engine::capture(card1.actor.clone(), |a| {
        a.set_size(vec2![100, 100]);
        a.set_position(Engine::get_world_center());
        a.set_color(Color::RED);
    });

    *player_card_value += card1.rank as i32;
    Engine::spawn(card1.clone().actor)?;

    let card2 = deck.draw_card();

    Engine::capture(card2.actor.clone(), |a| {
        a.set_size(vec2![100, 100]);
        // Offset any new cards by: CARD_OFFSET
        a.set_position(Engine::get_world_center() - SECOND_CARD_OFFSET);
        a.set_color(Color::BLUE);
    });

    *player_card_value += card2.rank as i32;
    Engine::spawn(card2.clone().actor)?;

    println!("Player has cards:\t{}\tand\t{}",
        card1.clone(),
        card2.clone()
    );

    // Generate dealer cards
    let dealer_card1 = deck.draw_card();

    Engine::capture(dealer_card1.actor.clone(), |a| {
        a.set_size(vec2![100, 100]);
        a.set_position(Engine::get_world_center() + vec2![60, -200]);
        a.set_color(Color::GREEN);
    });

    Engine::spawn(dealer_card1.clone().actor)?;

    let dealer_card2 = deck.draw_card();

    println!("Dealer has cards:\t{}\tand\t{}", 
        dealer_card1.clone(),
        dealer_card2.clone()
    );

    Engine::capture(dealer_card2.actor.clone(), |a| {
        a.set_size(vec2![100, 100]);
        a.set_position(Engine::get_world_center() + vec2![-60, -200]);
        a.set_color(Color::GREEN);
    });

    Engine::spawn(dealer_card2.actor)?;


    *dealer_card_value += dealer_card1.rank as i32;
    *dealer_card_value += dealer_card2.rank as i32;

    Ok(())
}

fn update() -> Result<(), RuntimeException> {

    let mut restart = false;

    {
        let mut round_over = load!("round_over", bool);
        if *round_over {
            println!("--- New Round ---");
            *round_over = false;
            restart = true;
        }

        drop!(round_over);
    } 

    if restart {    
        Engine::clear_scene()?;
        start()?;
        return Ok(());
    }

    let mut hit = load!("hit_signal", bool);
    let mut double_down = load!("double_down_signal", bool);
    let mut stand = load!("stand_signal", bool);
    let mut bust = load!("bust_signal", bool);

    let mut lock_hit = load!("lock_hit", bool);

    let mut player_card_value = load!("player_card_value", i32);

    if *hit && !*lock_hit && !*bust {
        // println!("player has hit!");

        let mut deck = load!("deck", Deck);
        let mut drawn_cards = load!("drawn_cards", i32);
        *drawn_cards += 1;
        
        if *drawn_cards < 9 && *player_card_value + 1 <= 21 { 
            let new_card = deck.draw_card();

            Engine::capture(new_card.actor.clone(), |a| {
                a.set_size(vec2![100, 100]);
                a.set_position(Engine::get_world_center() - nth_card_offset(*drawn_cards as f32));
                a.set_color(Color::ORANGE);
            });

            println!("Player has drawn: {}", new_card.clone());

            *player_card_value += new_card.rank as i32;
            if *player_card_value > 21 {
                *bust = true;
                println!("Player has busted!");
            } else {
                Engine::spawn(new_card.actor)?;
            }
        }

        *hit = false;
        drop!(deck, drawn_cards);
    }

    if *double_down && !*bust && !*lock_hit {
        println!("player has doubled down!");
        
        let mut deck = load!("deck", Deck);
        let drawn_cards = load!("drawn_cards", i32);
        let new_card = deck.draw_card();

        Engine::capture(new_card.actor.clone(), |a| {
            a.set_size(vec2![100, 100]);
            a.set_position(Engine::get_world_center() - nth_card_offset(*drawn_cards as f32));
            a.set_color(Color::ORANGE);
        });

        println!("Player has drawn: {}", new_card.clone());

        *player_card_value += new_card.rank as i32;
        if *player_card_value > 21 {
            *bust = true;
            println!("Player has busted!");
        } else {
            Engine::spawn(new_card.actor)?;
        }

        *lock_hit = true;
        *double_down = true;

        make_dealer_move(player_card_value.clone(), *bust);
        drop!(deck, drawn_cards);

    }

    if *stand && !*bust {
        println!("player stands!");
        make_dealer_move(player_card_value.clone(), false);

        *lock_hit = true;
        *stand = false;
    }

    if *bust && !*lock_hit {
        *lock_hit = true;
        make_dealer_move(player_card_value.clone(), true);
    }

    Ok(())
}

fn make_dealer_move(player_card_value: i32, player_bust: bool) {
    let mut dealer_card_value = load!("dealer_card_value", i32);
    let mut deck = load!("deck", Deck);
    let mut player_wins = load!("player_wins", i32);
    let mut dealer_wins = load!("dealer_wins", i32);

    // If player already busted -> dealer wins immediately
    if player_bust || player_card_value > 21 {
        println!("Player busted! Dealer wins.");
        *dealer_wins += 1;

        let mut round_over = load!("round_over", bool);
        *round_over = true;
        return;
    }

    // Dealer hits to 16
    while *dealer_card_value <= 16 {
        let new_card = deck.draw_card();

        Engine::capture(new_card.actor.clone(), |card| {
            card.set_size(vec2![100, 100]);
            card.set_color(Color::MAGENTA);
            card.set_position(Engine::get_world_center());
        });

        *dealer_card_value += new_card.rank as i32;

        if *dealer_card_value > 21 {
            println!("Dealer busted!");
            *player_wins += 1;

            let mut round_over = load!("round_over", bool);
            *round_over = true;
            return;
        }
    }

    // Only compare if neither busted
    match (*dealer_card_value).cmp(&player_card_value) {
        Ordering::Equal => {
            println!("Push!");
            *dealer_wins += 1;
            *player_wins += 1;
        }
        Ordering::Greater => {
            println!("Dealer wins!");
            *dealer_wins += 1;
        }
        Ordering::Less => {
            println!("Player wins!");
            *player_wins += 1;
        }
    }

    let mut round_over = load!("round_over", bool);
    *round_over = true;
}