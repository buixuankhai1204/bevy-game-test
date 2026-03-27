use bevy::prelude::*;
use crate::plugins::card_plugin::card::{Card, CardManager, CardType};

pub struct CardUIPlugin;

impl Plugin for CardUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_card_ui)
           .add_system(handle_card_interactions);
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
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        }).insert(Card { card_type: match i {
            0 => CardType::Power(1.5),
            1 => CardType::Luck { multiplier: 2.0, count: 3 },
            _ => CardType::WeatherControl(Vec2::new(0.0, 1.0)),
        } });
    }
}

fn handle_card_interactions(
    mut interaction_query: Query<(&Interaction, &Card, Entity), (Changed<Interaction>, With<Button>)>,
    mut card_manager: ResMut<CardManager>,
) {
    for (interaction, card, entity) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                card_manager.add_card(card.clone());
                info!("Card added to selection: {:?}", card.card_type);
            }
            _ => {}
        }
    }
}