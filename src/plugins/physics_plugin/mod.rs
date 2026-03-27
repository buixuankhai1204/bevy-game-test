pub mod projectile;
pub mod collision;
pub mod physics;
pub mod effects;
pub mod reflection;

use bevy::prelude::*;
use projectile::{Projectile, update_projectile};
use collision::handle_collisions;
use physics::calculate_velocity;
use effects::EffectsPlugin;
use reflection::calculate_bounce;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Vec2::new(0.0, -9.8))
           .add_system(update_projectile)
           .add_system(handle_collisions)
           .add_plugin(EffectsPlugin);
    }
}