use bevy::core_pipeline;
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(Camera2dBundle::default());

    // Add a start button
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                height: Val::Px(200.0),
                width: Val::Px(80.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color:Color::srgba(0.15, 0.15, 0.15, 1.0),
            border_color:Color::srgba(0.0,0.0,0.0,1.0),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Start",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

pub fn handle_button_click(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *BackgroundColor = Color::srgba(0.35, 0.75, 0.35, 1.0); // Change button color on click
                println!("Start button clicked!");
            }
            Interaction::Hovered => {
                *BackgroundColor = Color::srgba(0.25, 0.25, 0.25, 1.0); // Change color on hover
            }
            Interaction::None => {
                *BackgroundColor = Color::srgba(0.15, 0.15, 0.15, 1.0); // Default button color
            }
        }
    }
}
