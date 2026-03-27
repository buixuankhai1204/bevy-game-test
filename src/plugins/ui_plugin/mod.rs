pub mod turn_manager;

use bevy::prelude::*;
use turn_manager::{TurnManager, GameState};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TurnManager::new(10))
           .add_systems(Update, turn_management_system);

    }
}

fn turn_management_system(mut game_state: ResMut<GameState>) {
    // To access the inner GameState for methods expecting &GameState or &mut GameState,
    // deref or use as_ref()/as_mut().
    if TurnManager::is_game_over(game_state.as_ref()) {
        let winner = TurnManager::calculate_winner(game_state.as_ref());
        info!("Game Over: {}", winner);
    } else {
        TurnManager::next_turn(game_state.as_mut());
        info!("Next Turn: {:?}", game_state.current_turn);
    }
}