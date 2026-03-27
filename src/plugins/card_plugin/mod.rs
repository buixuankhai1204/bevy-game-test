pub mod card;
pub mod ui;

use bevy::prelude::*;
use ui::CardUIPlugin;
use card::{CardManager, CardType, Card, apply_card_effects};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardManager::new())
           .add_plugin(CardUIPlugin)
           .add_system(apply_card_effects_system);
    }
}

pub fn apply_card_effects_system(
    mut card_manager: ResMut<CardManager>,
    mut wind: ResMut<crate::plugins::map_plugin::weather::WindCondition>,
    mut commands: Commands,
    terrain_query: Query<(Entity, &mut crate::plugins::map_plugin::terrain::Terrain)>,
) {
    if let Some(card) = card_manager.draw_card() {
        apply_card_effects(card.card_type, wind, commands, terrain_query);
    }
}