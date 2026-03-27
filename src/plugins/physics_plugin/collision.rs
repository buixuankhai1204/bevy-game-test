use bevy::prelude::*;
use crate::plugins::map_plugin::terrain::{Terrain, TerrainMaterial};
use crate::plugins::physics_plugin::reflection::calculate_bounce;

pub fn handle_collisions(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut crate::plugins::physics_plugin::projectile::Projectile, &Transform)>,
    terrain_query: Query<(&Transform, &Terrain)>,
) {
    for (projectile_entity, mut projectile, projectile_transform) in projectile_query.iter_mut() {
        for (terrain_transform, terrain) in terrain_query.iter() {
            let distance = projectile_transform.translation - terrain_transform.translation;
            if distance.length() < 1.0 {
                match terrain.material {
                    TerrainMaterial::Sand => {
                        commands.entity(projectile_entity).despawn();
                    }
                    TerrainMaterial::Rock => {
                        let normal = Vec2::new(0.0, 1.0); // Placeholder normal
                        projectile.velocity = calculate_bounce(projectile.velocity, normal);
                    }
                    TerrainMaterial::Mud => {
                        projectile.velocity = Vec2::ZERO;
                    }
                }
            }
        }
    }
}