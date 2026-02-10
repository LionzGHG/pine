
use pine::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    R1, R2, R3, R4, R5, R6, R7, R8, R9,
    T, J, D, K, A
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

#[derive(Debug, Clone)]
pub struct Card {
    id: String,
    suit: Suit,
    rank: Rank,
    texture: Texture2D,
    transform: Transform,
}

impl Card {
    pub fn new(id: impl Into<String>, suit: Suit, rank: Rank, texture_name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            suit,
            rank,
            transform: Transform::default(),
            texture: Assets::get::<Texture2D>(texture_name.into().as_str()).unwrap(),
        }
    }
}

impl Component for Card {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new((*self).clone())
    }
    fn component_id(&self) -> String {
        self.id.clone()
    }

    fn init(&self, handle: &mut pine::Handle) {
        
    }

    fn render(&mut self, renderer: &mut pine::Renderer) {
        
    }
}