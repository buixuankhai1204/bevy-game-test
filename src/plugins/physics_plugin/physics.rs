use bevy::prelude::*;

pub fn calculate_velocity(angle: f32, power: f32) -> Vec2 {
    let radians = angle.to_radians();
    Vec2::new(
        power * radians.cos(),
        power * radians.sin(),
    )
}

pub fn update_projectile(mut query: Query<(&mut Transform, &mut crate::plugins::physics_plugin::projectile::Projectile)>, time: Res<Time>, gravity: Res<Vec2>) {
    for (mut transform, mut projectile) in query.iter_mut() {
        projectile.velocity.y -= gravity.y * time.delta_seconds();
        transform.translation += projectile.velocity.extend(0.0) * time.delta_seconds();
    }
}