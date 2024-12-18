use bevy::core_pipeline;
use bevy::prelude::*;
use bevy::time::common_conditions::paused;
use crate::game_logic::*;
use crate::{game_logic, players};
use crate::players::{Player, PlayerName, PlayerStatus};


pub fn setup_ui(mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut game_query: Query<&mut Game>,) {

    let mut background_color: BackgroundColor =  if let Ok(mut game) = game_query.get_single_mut() {
        if *game.get_state() == GameState::Paused {
            BackgroundColor(Color::srgba(1.0, 0.0, 0.60, 1.0))
        }else{
            BackgroundColor(Color::srgba(0.0, 1.0, 0.60, 1.0))
        }
    } else {
        println!("Could not find a single game component!");
        BackgroundColor(Color::srgba(0.0, 1.0, 0.60, 1.0)) // Default or fallback color
    };
    // Add a start button
    commands
        .spawn(ButtonBundle {
            style: Style {
                height: Val::Px(100.0),
                width: Val::Px(150.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                top: Val::Px(300.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            background_color: background_color,
            border_color: BorderColor(Color::srgba(0.0,0.0,0.0,1.0)),
            border_radius: BorderRadius::new(
                Val::Px(10.),
                // top right
                Val::Px(10.),
                // bottom right
                Val::Px(10.),
                // bottom left
                Val::Px(10.),
            ),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Start/Pause",
                    TextStyle {
                        font: asset_server.load("fonts/MountainsofChristmas-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });
        });
}

pub fn handle_button_click(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>)>,
    mut event_writer: EventWriter<game_logic::ButtonPressedEvent>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgba(0.35, 0.75, 0.35, 1.0)); // Change button color on click
                println!("Start button clicked!");
                // Access the game component mutably
               event_writer.send(game_logic::ButtonPressedEvent);

            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.25, 0.25, 0.25, 1.0)); // Change color on hover
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 1.0)); // Default button color
            }
        }
    }
}
