
pub use crate::prelude::*;


pub const SECOND_CARD_OFFSET: Vec2 = Vec2::new_f32(0., -25.);

// TEXTURE for hidden cards
pub const CARD_HIDDEN: &str = "";

pub const fn nth_card_offset(cards_drawn: f32) -> Vec2 {
    Vec2::new_f32(0., SECOND_CARD_OFFSET.y - (-SECOND_CARD_OFFSET.y * cards_drawn))
}

#[derive(Clone)]
pub struct Deck(pub Vec<Card>);

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::<Card>::with_capacity(32);
        let mut index = 1i32;

        for suit in Suit::all() {
            for rank in Rank::all() {
                cards.push(
                    Card::new(
                        &format!("Card{index}"),
                        rank,
                        suit
                    )
                );

                index += 1;
            }
        }

        let mut new_deck = Deck(cards);
        new_deck.shuffle();
        new_deck
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.0.clone()
    }

    pub fn shuffle(&mut self) {
        // Use fisher-yates-shuffle algorithm
        for i in (1..self.0.len()).rev() {
            let j = rand::random_range(0..=i);
            self.0.swap(i, j);
        }
    }

    pub fn draw_card(&mut self) -> Card {
        if let Some(card) = self.0.pop() {
            card
        } else {
            self.rebuild();
            self.shuffle();
            self.draw_card()
        }
    }

    pub fn rebuild(&mut self) {
        self.0 = Deck::new().0;
    }
}

#[derive(Clone)]
pub struct Card {
    pub id: String,
    pub actor: Rc<RefCell<Actor>>,
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(id: &str, rank: Rank, suit: Suit) -> Card {
        let actor = make!(Actor::new(id, CARD_HIDDEN));
        
        Engine::capture(actor.clone(), |a| {
            a.add_attribute(rank, format!("{id}_Rank"));
            a.add_attribute(suit, format!("{id}_Suit"));
        });

        Card {
            id: id.to_string(),
            actor: actor,
            suit,
            rank
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  {}", self.suit, self.rank)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    R1, R2, R3, R4, R5, R6, R7, R8, R9, T, J, Q, K, A
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::R1 => "1",
            Self::R2 => "2",
            Self::R3 => "3",
            Self::R4 => "4",
            Self::R5 => "5",
            Self::R6 => "6",
            Self::R7 => "7",
            Self::R8 => "8",
            Self::R9 => "9",
            Self::T  => "T",
            Self::J  => "J",
            Self::Q  => "Q",
            Self::K  => "K",
            Self::A  => "A",
        })
    }
}

impl Rank {
    pub fn all() -> [Rank; 14] {
        [
            Rank::R1, Rank::R2, Rank::R3, Rank::R4, Rank::R5,
            Rank::R6, Rank::R7,
            Rank::R8, Rank::R9, Rank::T, Rank::J, Rank::Q, Rank::K, Rank::A
        ]
    }
}

impl Into<i32> for Rank {
    fn into(self) -> i32 {
        match self {
            Self::R1 => 1,
            Self::R2 => 2,
            Self::R3 => 3,
            Self::R4 => 4,
            Self::R5 => 5,
            Self::R6 => 6,
            Self::R7 => 7,
            Self::R8 => 8,
            Self::R9 => 9,
            Self::T | Self::J | Self::Q | Self::K | Self::A => 10,
        }
    }
}

impl Attribute for Rank {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Diamonds, Hearts, Spades, Clubs
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Diamonds  => "♦️",
            Self::Hearts    => "♥️",
            Self::Spades    => "♠️",
            Self::Clubs     => "♣️",
        })
    }
}

impl Suit {
    pub fn all() -> [Suit;4] {
        [Suit::Diamonds, Suit::Hearts, Suit::Spades, Suit::Clubs]
    }
}

impl Attribute for Suit {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
