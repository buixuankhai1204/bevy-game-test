use bevy::prelude::*;
use crate::plugins::map_plugin::terrain::{Terrain, TerrainMaterial};

pub fn handle_collisions(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<crate::plugins::physics_plugin::projectile::Projectile>>,
    terrain_query: Query<(&Transform, &Terrain)>,
) {
    for (projectile_entity, projectile_transform) in projectile_query.iter() {
        for (terrain_transform, terrain) in terrain_query.iter() {
            let distance = projectile_transform.translation - terrain_transform.translation;
            if distance.length() < 1.0 {
                match terrain.material {
                    TerrainMaterial::Sand => {
                        // Absorb projectile in sand
                        commands.entity(projectile_entity).despawn();
                    }
                    TerrainMaterial::Rock => {
                        // Bounce logic (placeholder)
                        info!("Projectile bounced on rock");
                    }
                    TerrainMaterial::Mud => {
                        // Stick projectile to mud
                        info!("Projectile stuck in mud");
                    }
                }
            }
        }
    }
}