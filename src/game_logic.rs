use bevy::prelude::*;
use rand::seq::SliceRandom;
use bevy::app::AppExit;
use crate::{game_logic, players};
use crate::players::*;
use std::thread;
use std::time::Duration;
pub struct GameLogicPlugin;
#[derive(Event)]
pub struct SelectPlayer;

#[derive(Event)]
pub struct SelectRecipient;


#[derive(PartialEq, Event)]
pub struct ButtonPressedEvent;
#[derive(Debug, PartialEq)]
pub enum GameState {
    Idle,
    Running,
    Paused,
    GameOver,
}

#[derive(Component)]
pub struct Game {
    state: GameState,
}

impl Game {
    pub fn new() -> Game{
        Game {
            state: GameState::Idle,
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }
    pub fn pause_game(&mut self) {
        self.state = GameState::Paused;
    }
    pub fn next_state(&mut self){

        match self.state{
            GameState::Idle => self.state = GameState::Running,
            GameState::Running => self.state = GameState::Paused,
            GameState::Paused => self.state = GameState::Running,
            GameState::GameOver => println!("Game over, re-open application."),
        }
        println!("Game state is {:?}", self.state);
    }
    pub fn game_over(player_query: Query<&Player>,
                     mut game_query: Query<&mut Game>,
                     mut event_writer: EventWriter<AppExit>,
    ){
        if let Ok(mut game) = game_query.get_single_mut() {
            game.pause_game();
        } else {
            println!("Could not find a single game component!");
        }

        for player in player_query.iter() {
            if player.get_tickets() > 0 {
                return
            }
        }
            println!("game is over  now");
            event_writer.send(AppExit::Success);



    }
}

pub fn new_game(mut commands: Commands){
    commands.spawn(Game::new());
}

pub fn on_button_pressed(
    mut event_reader: EventReader<ButtonPressedEvent>,
    mut new_player_event: EventWriter<SelectPlayer>,
    mut new_recipient_event: EventWriter<SelectRecipient>,
){
    for event in event_reader.read() {
        if *event == ButtonPressedEvent {
            println!("\n new players selected\n");
            new_player_event.send(SelectPlayer);
           // thread::sleep(Duration::from_secs(2));
            new_recipient_event.send(SelectRecipient);
        }
    }
}