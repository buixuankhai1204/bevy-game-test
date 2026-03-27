pub mod terrain;

use bevy::prelude::*;
use terrain::{Terrain, TerrainMaterial};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_terrain);
    }
}

fn setup_terrain(mut commands: Commands) {
    commands.spawn().insert(Terrain {
        material: TerrainMaterial::Rock,
    });
}