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

    pub fn show_scores_per_frame(&self) {
        for frame in 1..=Game::MAX_FRAMES_PER_GAME as usize {
            let frame_number = frame - 1;

            for (p_idx, player) in self.players.iter().enumerate() {
                match player.0.get(frame_number) {
                    Some(f) => {
                        println!("Player {}, Frame {} => {:?}",
                            p_idx + 1, f.frame, f.rolls)
                    },
                    _ => continue
                }
            }
        }

        // @TODO Show the final scores for each player
    }

    pub fn show_scores_per_player(&self) {
        for (idx, player) in self.players.iter().enumerate() {
            println!("Player {} Frame Scores", idx);

            for frame in player.0.iter() {
                println!("{:?}", frame);
            }

            // @TODO Show the player's final score
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
