use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayerTurn {
    Player1,
    Player2,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct GameState {
    pub current_turn: PlayerTurn,
    pub player1_score: i32,
    pub player2_score: i32,
    pub max_turns: u32,
    pub current_turn_number: u32,
}

pub struct TurnManager;

impl TurnManager {
    pub fn new(max_turns: u32) -> GameState {
        GameState {
            current_turn: PlayerTurn::Player1,
            player1_score: 0,
            player2_score: 0,
            max_turns,
            current_turn_number: 1,
        }
    }

    pub fn next_turn(game_state: &mut GameState) {
        if game_state.current_turn_number >= game_state.max_turns {
            return;
        }

        match game_state.current_turn {
            PlayerTurn::Player1 => game_state.current_turn = PlayerTurn::Player2,
            PlayerTurn::Player2 => {
                game_state.current_turn = PlayerTurn::Player1;
                game_state.current_turn_number += 1;
            }
        }
    }

    pub fn is_game_over(game_state: &GameState) -> bool {
        game_state.current_turn_number >= game_state.max_turns
    }

    pub fn calculate_winner(game_state: &GameState) -> &'static str {
        if game_state.player1_score > game_state.player2_score {
            "Player 1 Wins!"
        } else if game_state.player1_score < game_state.player2_score {
            "Player 2 Wins!"
        } else {
            "Draw!"
        }
    }
}