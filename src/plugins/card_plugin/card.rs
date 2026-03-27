use bevy::prelude::*;
use crate::plugins::physics_plugin::physics::calculate_velocity;
use crate::plugins::map_plugin::weather::WindCondition;
use crate::plugins::map_plugin::terrain::TerrainMaterial;

#[derive(Debug, Clone)]
pub enum CardType {
    Power(f32),       // Tăng sát thương, đá mạnh hơn.
    Luck { multiplier: f32, count: u32 }, // Xác suất x2 sát thương hoặc bắn ra nhiều viên đá.
    WeatherControl(Vec2), // Thay đổi hướng gió.
    TerrainMod { material: TerrainMaterial }, // Biến mặt đất thành bùn hoặc đá.
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

pub fn apply_card_effects(
    card: CardType,
    mut wind: ResMut<WindCondition>,
    mut commands: Commands,
    query: Query<(Entity, &mut crate::plugins::map_plugin::terrain::Terrain)>,
) {
    match card {
        CardType::Power(scalar) => {
            info!("Applied Power card with scalar {}", scalar);
            // Placeholder logic
        }
        CardType::Luck { multiplier, count } => {
            info!("Applied Luck card: multiplier {}, count {}", multiplier, count);
        }
        CardType::WeatherControl(new_direction) => {
            wind.direction = new_direction;
            info!("WeatherControl card applied. New wind direction: {:?}", wind.direction);
        }
        CardType::TerrainMod { material } => {
            for (entity, mut terrain) in query.iter() {
                terrain.material = material.clone();
                commands.entity(entity).insert(terrain);
            }
            info!("Terrain modified with new material");
        }
    }
}