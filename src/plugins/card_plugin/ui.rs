use bevy::prelude::*;

pub struct CardUIPlugin;

impl Plugin for CardUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_card_ui)
           .add_system(handle_card_clicks);
    }
}

fn setup_card_ui(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_material = materials.add(Color::rgb(0.25, 0.25, 0.75).into());

    for i in 0..3 {
        commands.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Px(10.0)),
                ..Default::default()
            },
            material: button_material.clone(),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    format!("Card {}", i + 1),
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
    }
}

fn handle_card_clicks(mut interaction_query: Query<(&Interaction, &mut Style), (Changed<Interaction>, With<Button>)>) {
    for (interaction, mut style) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                style.size = Size::new(Val::Px(160.0), Val::Px(75.0));
                info!("Card Clicked");
            }
            Interaction::Hovered => {
                style.size = Size::new(Val::Px(155.0), Val::Px(70.0));
            }
            Interaction::None => {
                style.size = Size::new(Val::Px(150.0), Val::Px(65.0));
            }
        }
    }
}