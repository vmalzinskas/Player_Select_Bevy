use bevy::prelude::*;
use crate::players;
use crate::players::*;


#[derive(Component)]
struct NameTag;

pub fn spawn_name_tag(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    parent_entity: Entity,
    player_name: &PlayerName,
    player_status: &PlayerStatus,
    ) {

    let background_color = if *player_status == players::PlayerStatus::Current {
        // Green for `playerStatus::Current`
        BackgroundColor(Color::srgba(0.0, 1.0, 0.60, 1.0))
    }else if *player_status == players::PlayerStatus::Recipient {
        // blue for 'playerStatus::Recipient
        BackgroundColor(Color::srgba(0.0, 0.1, 1.0, 1.0))
    } else {
        // Default color
        BackgroundColor(Color::srgba(0.7, 0.3, 0.1, 1.0))
    };

    commands.entity(parent_entity).with_children(|parent|{
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(200.0), // Box dimensions
                    height: Val::Px(100.0),
                    margin: UiRect::all(Val::Px(5.0)),              // Margin around the box
                    justify_content: JustifyContent::Center,        // Center text horizontal
                    align_items: AlignItems::Center,                // Center text vertical
                    ..default()
                },
                background_color,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        player_name.0.clone(),
                        TextStyle {
                            font: asset_server.load("fonts/MountainsofChristmas-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                    ..default()
                });
            })
            .insert(NameTag);
    });
}

pub fn spawn_multiple_name_tags(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &PlayerName, &PlayerStatus), With<Player>>,
) {
    let container =  commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                //flex_direction: FlexDirection::Column, // Arrange children vertically
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::SpaceEvenly, // Spread children evenly
                align_items: AlignItems::Center, // Center items horizontally
                width: Val::Percent(100.0), // Container takes full width
                height: Val::Percent(100.0), // Container takes full height
                column_gap: Val::Px(10.0), // Spacing between col items
                row_gap: Val::Px(10.0), // Spacing between row items
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        })
        .id();

    for (_entity, player_name, player_status) in query.iter() {
        spawn_name_tag(&mut commands, &asset_server, container, player_name, player_status);
    }
}
