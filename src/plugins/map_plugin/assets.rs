use bevy::prelude::*;

pub struct TerrainAssets {
    pub sand_texture: Handle<Texture>,
    pub rock_texture: Handle<Texture>,
    pub mud_texture: Handle<Texture>,
}

impl FromWorld for TerrainAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        TerrainAssets {
            sand_texture: asset_server.load("textures/sand.png"),
            rock_texture: asset_server.load("textures/rock.png"),
            mud_texture: asset_server.load("textures/mud.png"),
        }
    }
}