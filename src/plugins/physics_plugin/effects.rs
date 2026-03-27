use bevy::prelude::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_collision_effects);
    }
}

fn spawn_collision_effects(mut commands: Commands, events: Res<Events<CollisionEvent>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for event in events.iter() {
        if let CollisionEvent::Started(entity_a, entity_b, _manifold) = event {
            // Spawning particle effect at collided position (placeholder logic)
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(1.0, 0.5, 0.0).into()),
                ..Default::default()
            });
        }
    }
}