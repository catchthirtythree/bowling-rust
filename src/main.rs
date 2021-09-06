#![allow(dead_code)]
#![allow(unused_variables)]

mod frame;
mod game;
mod player;
mod scoring;

use crate::frame::Frame;
use crate::game::{Game, GameState};
use crate::scoring::Scoring;

const MAX_FRAMES_PER_GAME: usize = 10;
const MAX_ROLL_SCORE: u32 = 10;

fn main() {
    let mut game = Game::new();

    game.add_player(String::from("Player 1"));
    game.add_player(String::from("Player 2"));

    match game.start() {
        Ok(_) => {
            while game.state != GameState::Done {
                game.play();
            }
        },

        Err(e) => println!("{:?}", e)
    }

    for player in game.players {
        println!("Stats for {}", player.name);

        for frame in &player.frames {
            println!("Frame {} => {:?}", frame.number, frame.rolls);
        }

        println!("Final Score for {} => {}", player.name,
            Scoring::score_frames(&player.frames));
    }
}
