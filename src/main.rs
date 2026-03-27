use bevy::prelude::*;

mod plugins {
    pub mod map_plugin;
    pub mod player_plugin;
    pub mod physics_plugin;
    pub mod card_plugin;
    pub mod ui_plugin;
}

use plugins::{
    map_plugin::MapPlugin,
    player_plugin::PlayerPlugin,
    physics_plugin::PhysicsPlugin,
    card_plugin::CardPlugin,
    ui_plugin::UIPlugin,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Menu,
    CardSelection,
    PlayerTurn,
    ProjectileFlying,
    RoundEnd,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Menu)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(CardPlugin)
        .add_plugin(UIPlugin)
        .run();
}