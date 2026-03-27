pub mod turn_manager;

use bevy::prelude::*;
use turn_manager::{TurnManager, GameState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TurnManager::new(10))
           .add_system(turn_management_system);
    }
}

fn turn_management_system(mut game_state: ResMut<GameState>) {
    if TurnManager::is_game_over(&game_state) {
        let winner = TurnManager::calculate_winner(&game_state);
        info!("Game Over: {}", winner);
    } else {
        TurnManager::next_turn(&mut game_state);
        info!("Next Turn: {:?}", game_state.current_turn);
    }
}