use bevy::prelude::*;
use rand::seq::SliceRandom;
use bevy::prelude::{Resource, Timer};
use bevy::utils::tracing::event;
use crate::game_logic;
use crate::game_logic::*;

#[derive(Resource)]
pub struct PlayerTimer( pub Timer);

#[derive(Component, Debug, PartialEq)]
pub enum PlayerStatus {
    Idle,
    Recipient,
    Current,
}

#[derive(Component)]
pub struct Player {
    tickets: u32,
}
impl Player{
    pub fn get_tickets(&self) -> u32 {self.tickets}
}
#[derive(Component, Clone)]
pub struct PlayerName(pub String);

pub fn add_people(mut commands: Commands){
    commands.spawn((Player{tickets: 3}, PlayerName("Blair".to_string()), PlayerStatus::Idle));
    commands.spawn((Player{tickets: 3}, PlayerName("Tess".to_string()), PlayerStatus::Idle));
    commands.spawn((Player{tickets: 3}, PlayerName("Ross".to_string()), PlayerStatus::Idle));
    commands.spawn((Player{tickets: 3}, PlayerName("Bonnie".to_string()), PlayerStatus::Idle));
    //commands.spawn((Player{tickets: 3}, PlayerName("Max".to_string()), PlayerStatus::Idle));
    //commands.spawn((Player{tickets: 3}, PlayerName("Heidi".to_string()), PlayerStatus::Idle));
    // commands.spawn((Player{tickets: 3}, PlayerName("Player 7".to_string()), PlayerStatus::Idle));
    // commands.spawn((Player{tickets: 3}, PlayerName("Player 8".to_string()), PlayerStatus::Idle));
    // commands.spawn((Player{tickets: 3}, PlayerName("Player 9".to_string()), PlayerStatus::Idle));
    // commands.spawn((Player{tickets: 3}, PlayerName("Player 10".to_string()), PlayerStatus::Idle));
}
pub fn debug_people(query: Query<(&PlayerName, &PlayerStatus, &Player)>){
    println!("Debug player");
    for (name, status, player) in query.iter(){
        println!{"{} is {:?} and has {} tickets.", name.0, status, player.get_tickets()};
    }
}
pub fn select_recipient_player(
    mut query: Query<(&mut PlayerStatus, &mut Player)>,
    mut event_reader: EventReader<game_logic::SelectRecipient>,
) {
    for event in event_reader.read(){
        println!("select_random_idle_player");
        // Collect all idle players
        let mut idle_players: Vec<(Mut<PlayerStatus>, Mut<Player>)> = query
            .iter_mut()
            .filter_map(|(mut status, mut player)| {
                if *status == PlayerStatus::Idle && player.get_tickets() > 0 {
                    Some((status, player))
                } else if *status == PlayerStatus::Recipient {
                    *status = PlayerStatus::Idle;
                    None
                } else {
                    None
                }
            })
            .collect();

        // Randomly select one
        let mut rng = rand::thread_rng();
        if let Some((ref mut status, ref mut player)) = idle_players.choose_mut(&mut rng) {
            // Update the selected player to `Current`
            **status = PlayerStatus::Recipient;
            player.tickets -= 1;
            println!("Recipient player: select");
        } else {
            println!("No idle players available.");
        }
    }
}

pub fn select_random_idle_player(
    mut query: Query<(&mut PlayerStatus, &PlayerName), With<Player>>,
    mut event_reader: EventReader<game_logic::SelectPlayer>,
) {
    for event in event_reader.read(){
        println!("select_random_idle_player");
        // Collect all idle players
        let mut idle_players: Vec<(Mut<PlayerStatus>, &PlayerName)> = query
            .iter_mut()
            .filter_map(|(mut status, name)| {
                if *status == PlayerStatus::Idle {
                    Some((status, name))
                } else if *status == PlayerStatus::Current {
                    *status = PlayerStatus::Idle;
                    None
                } else {
                    None
                }
            })
            .collect();

        // Randomly select one
        let mut rng = rand::thread_rng();
        if let Some((ref mut status, name)) = idle_players.choose_mut(&mut rng) {
            // Update the selected player to `Current`
            **status = PlayerStatus::Current;
            println!("Selected player: {}", name.0);
        } else {
            println!("No idle players available.");
        }
    }
}

pub fn start_players(query: Query<&PlayerStatus>){
    let idle_players: Vec<&PlayerStatus> = query
        .iter()
        .collect();

    let mut rng= rand::thread_rng();
    if let Some(first_player) = idle_players.choose(&mut rng) {
        // Use `first_player`
    } else {
        println!("No players available.");
    }
}
pub fn get_player_rand(query: Query<(
    &Player,
    &PlayerName,
    &PlayerStatus)>
) -> Option<PlayerName> {
    let idle_players: Vec<PlayerName> = query
        .iter()
        .filter_map(|(_, player_name, status)| {
            if let PlayerStatus::Idle = status {
                Some(player_name.clone()) // Clone PlayerName for ownership
            } else {
                None
            }
        })
        .collect();

    let mut rng= rand::thread_rng();
    idle_players.choose(&mut rng).cloned()
}

pub fn get_idle_players(query: Query<(
    &Player,
    &PlayerName,
    &PlayerStatus)>
) -> Option<PlayerName> {
    let players: Vec<PlayerName> = query
        .iter()
        .map(|(_, player_name, _)| player_name.clone()) // Clone PlayerName for ownership
        .collect();

    let mut rng= rand::thread_rng();
    players.choose(&mut rng).cloned()
}

pub fn current_to_idle(mut query: Query<(&mut PlayerStatus, &PlayerName), With<Player>>) {
    for mut status in query.iter_mut() {
        if *status.0 == PlayerStatus::Current {
            *status.0 = PlayerStatus::Idle;
        }
    }
}
pub fn get_current_player_name(query: Query<(
    &Player,
    &PlayerName,
    &PlayerStatus)>
) -> Result<String, String> {
    for player in query.iter() {
        if *player.2 == PlayerStatus::Current {
            return Ok(player.1.0.clone());
        }
    }
    Err("No current player found.".to_string())
}