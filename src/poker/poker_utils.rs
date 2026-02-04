
use pine::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, T, J, Q, K, A
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

impl Attribute for Suit {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct Card(pub Rc<RefCell<Actor>>);

impl Card {
    pub fn new(id: &str, texture_path: &str, rank: Rank, suit: Suit) -> Card {
        let base_component = make!(Actor::new(id, texture_path));
        get!(base_component).add_attribute(rank, format!("{}_rank", id));
        get!(base_component).add_attribute(suit, format!("{}_suit", id));

        Card(base_component)
    }
}