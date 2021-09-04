use crate::frame::Frame;
use crate::player::Player;
use crate::scoring::Scoring;

pub struct Game {
    players: Vec<Player>,
}

impl Game {
    pub const MAX_FRAMES_PER_GAME: usize = 10;
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
                            p_idx + 1, f.number, f.rolls)
                    },
                    _ => continue
                }
            }
        }

        println!();

        for (p_idx, player) in self.players.iter().enumerate() {
            println!("Player {}, Final Score => {}",
                p_idx + 1, Scoring::score_frames(&player.0));
        }
    }

    pub fn show_scores_per_player(&self) {
        for (idx, player) in self.players.iter().enumerate() {
            println!("Player {} Frame Scores", idx + 1);

            for frame in player.0.iter() {
                println!("{:?}", frame);
            }

            println!("Player {}, Final Score => {}",
                idx + 1, Scoring::score_frames(&player.0));
        }
    }

    pub fn simulate_game(&mut self) {
        for current_frame in 1..=Game::MAX_FRAMES_PER_GAME {
            for player in self.players.iter_mut() {
                player.0.push(Frame::roll_frame(current_frame));
            }
        }
    }
}
