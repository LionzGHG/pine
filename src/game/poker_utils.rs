
use pine::prelude::*;

pub struct Player {
    pub player_id: String,
    pub cards: [Card; 2],
}

impl Player {
    pub fn new(player_id: &str, commands: &Commands) -> Player {
        let card1 = Card::new(
            format!("{}_card1", player_id).as_str(),
            "King_Spades",
            Rank::K,
            Suit::Spades
        );

        get!(card1.0).transform.set_position(
            Vec2D::new(commands.half_width() / 4, commands.half_height() / 2)
        );
        get!(card1.0).transform.scale = 0.2;

        let card2 = Card::new(
            format!("{}_card2", player_id).as_str(),
            "King_Spades",
            Rank::K,
            Suit::Spades
        );

        get!(card2.0).transform.set_position(
            Vec2D::new(commands.half_width() / 2, commands.half_height() / 2)
        );
        get!(card2.0).transform.scale = 0.2;

        Player {
            cards: [card1, card2],
            player_id: player_id.into()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, T, J, Q, K, A
}

impl Rank {
    pub fn all() -> [Rank; 14] {
        [
            Rank::R1, Rank::R2, Rank::R3, Rank::R4, Rank::R5, 
            Rank::R6, Rank::R7, 
            Rank::R8, Rank::R9,
            Rank::T, Rank::J, Rank::Q, Rank::K, Rank::A
        ]
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
pub enum Suit {
    Diamonds, Hearts, Spades, Clubs
}

impl Suit {
    pub fn all() -> [Suit; 4] {
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

#[derive(Clone)]
pub struct Card(pub Rc<RefCell<Actor>>);

impl Card {
    pub fn new(id: &str, texture_path: &str, rank: Rank, suit: Suit) -> Card {
        let base_component = make!(Actor::new(id, texture_path));
        get!(base_component).add_attribute(rank, format!("{}_rank", id));
        get!(base_component).add_attribute(suit, format!("{}_suit", id));

        Card(base_component)
    }
}