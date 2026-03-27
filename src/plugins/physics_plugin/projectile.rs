use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
}

pub struct ProjectileBundle {
    pub velocity: Vec2,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
        }
    }
}

pub fn update_projectile(mut query: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, projectile) in query.iter_mut() {
        transform.translation += projectile.velocity.extend(0.0) * time.delta_seconds();
    }
}