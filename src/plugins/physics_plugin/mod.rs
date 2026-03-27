pub mod projectile;
pub mod collision;

use bevy::prelude::*;
use projectile::{Projectile, update_projectile};
use collision::handle_collisions;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_projectile)
           .add_system(handle_collisions);
    }
}