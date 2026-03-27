use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum CardType {
    Power,       // Tăng sát thương, đá to hơn.
    Luck,        // Có tỉ lệ x2 sát thương hoặc ném 3 viên đá cùng lúc.
    WeatherControl, // Thay đổi hướng gió có lợi.
    TerrainMod,  // Biến mặt đất xung quanh đối thủ thành bùn.
}

#[derive(Component)]
pub struct Card {
    pub card_type: CardType,
}

pub struct CardManager {
    pub deck: Vec<Card>,
}

impl CardManager {
    pub fn new() -> Self {
        Self { deck: vec![] }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.deck.pop()
    }

    pub fn add_card(&mut self, card: Card) {
        self.deck.push(card);
    }
}