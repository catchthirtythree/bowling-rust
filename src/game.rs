use crate::frame::Frame;
use crate::player::Player;

pub struct Game {
    players: Vec<Player>,
}

impl Game {
    pub const MAX_FRAMES_PER_GAME: u32 = 10;
    pub const MAX_ROLL_SCORE: u32 = 10;

    pub fn new(players: Vec<Player>) -> Game {
        Game {
            players,
        }
    }

    pub fn show_scores(&self) {
        for player in self.players.iter() {
            for frame in player.0.iter() {
                println!("{:?}", frame);
            }
        }
    }

    pub fn simulate_game(&mut self) {
        for current_frame in 1..=Game::MAX_FRAMES_PER_GAME {
            for player in self.players.iter_mut() {
                let mut frame = Frame::new(current_frame);

                frame.roll_frame();
                player.0.push(frame);
            }
        }
    }
}
