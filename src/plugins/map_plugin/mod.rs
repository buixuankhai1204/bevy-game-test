pub mod terrain;
pub mod assets;
pub mod weather;

use bevy::prelude::*;
use terrain::{Terrain, TerrainMaterial};
use assets::TerrainAssets;
use weather::WeatherPlugin;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WeatherPlugin)
           .init_resource::<TerrainAssets>()
           .add_startup_system(setup_terrain);
    }
}

fn setup_terrain(mut commands: Commands, terrain_assets: Res<TerrainAssets>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let sand_material = materials.add(terrain_assets.sand_texture.clone().into());

    commands.spawn().insert(Terrain {
        material: TerrainMaterial::Sand,
    });
}