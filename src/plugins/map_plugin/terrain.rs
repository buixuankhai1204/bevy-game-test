use bevy::prelude::*;

#[derive(Component)]
pub struct Terrain {
    pub material: TerrainMaterial,
}

pub enum TerrainMaterial {
    Sand,   // Less bounce
    Rock,   // High bounce
    Mud,    // Sticky projectile
}