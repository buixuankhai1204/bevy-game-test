pub mod card;
pub mod ui;

use bevy::prelude::*;
use ui::CardUIPlugin;
use card::{CardManager, CardType, Card};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CardManager::new())
           .add_plugin(CardUIPlugin);
    }
}