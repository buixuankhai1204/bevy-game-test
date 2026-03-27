use bevy::prelude::*;

#[derive(Resource)]
pub struct WindCondition {
    pub direction: Vec2,
    pub strength: f32,
}

pub struct WeatherPlugin;

impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindCondition {
            direction: Vec2::new(1.0, 0.0),
            strength: 0.5,
        })
        .add_startup_system(setup_weather)
        .add_system(update_weather);
    }
}

fn setup_weather() {
    info!("Weather System initialized");
}

fn update_weather(mut wind_condition: ResMut<WindCondition>) {
    // Update logic for wind (direction/strength) can go here. Placeholder:
    wind_condition.strength += 0.01;
}