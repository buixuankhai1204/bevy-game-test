pub mod projectile;

use bevy::prelude::*;
use projectile::{Projectile, update_projectile};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_projectile);
    }
}