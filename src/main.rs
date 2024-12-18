mod nametag;
mod players;
mod user_interface;
mod game_logic;

use bevy::color::palettes::css::BLANCHED_ALMOND;
use bevy::prelude::*;
use crate::user_interface::handle_button_click;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<game_logic::ButtonPressedEvent>()
        .add_event::<game_logic::SelectPlayer>()
        .add_event::<game_logic::SelectRecipient>()
        .add_systems(Startup, (
            game_logic::new_game,
            setup_camera,
            players::add_people,
            players::select_random_idle_player,
            players::select_recipient_player,
            //nametag::spawn_multiple_name_tags.after(players::add_people),
            user_interface::setup_ui,
        ))
        .add_systems(Update, (
            user_interface::handle_button_click,
            nametag::spawn_multiple_name_tags.after(players::add_people),
            players::debug_people,
            game_logic::on_button_pressed,
            game_logic::Game::game_over,
            players::select_random_idle_player,
            players::select_recipient_player.after(players::select_random_idle_player),
        ))
        .run();
}

fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(BLANCHED_ALMOND.into()),
            ..default()
        },
        ..default()
    });
}
