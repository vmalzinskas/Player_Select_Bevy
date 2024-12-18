use bevy::prelude::*;

#[derive(Component)]
struct NameTag;

pub fn spawn_name_tag(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(200.0), // Box dimensions
                    height: Val::Px(100.0),
                    margin: UiRect::all(Val::Px(5.0)),              // Margin around the box
                    justify_content: JustifyContent::Center,        // Center text horizontal
                    align_items: AlignItems::Center,                // Center text vertical
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.7,0.3,0.1,1.0)),
                ..default()
            })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Player Name",
                    TextStyle {
                        font: asset_server.load("fonts/MountainsofChristmas-Bold.ttf"),
                        font_size: 15.0,
                        ..default()
                    },
                ),
                ..default()
            });
        })
        .insert(NameTag);
}