use crate::{MAX_FRAMES_PER_GAME};
use crate::frame::Frame;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub nickname: String,
    pub frames: Vec<Frame>,
}

impl Player {
    pub fn new(name: String) -> Player {
        let nickname = name.chars().take(3).collect();

        Player {
            name,
            nickname,
            frames: vec![]
        }
    }

    pub fn is_finished_game(&self) -> bool {
        self.is_finished_turn() && self.frames.len() == MAX_FRAMES_PER_GAME
    }

    pub fn is_finished_turn(&self) -> bool {
        match self.frames.last() {
            Some(frame) => frame.is_finished(),
            None => panic!("Player has no frames??")
        }
    }

    pub fn reset(&mut self) {
        self.frames = vec![];
    }

    pub fn roll(&mut self) {
        println!("Player {} is rolling.", self.name);

        match self.frames.last_mut() {
            Some(frame) => {
                if frame.is_finished() {
                    self.roll_new_frame()
                } else {
                    frame.roll();
                }
            },

            None => self.roll_new_frame()
        }
    }

    fn roll_new_frame(&mut self) {
        assert!(self.frames.len() != MAX_FRAMES_PER_GAME);

        let frame_number = self.frames.len() + 1;
        let mut next_frame = Frame::new(frame_number);
        next_frame.roll();
        self.frames.push(next_frame);
    }
}

impl Default for Player {
    fn default() -> Player {
        Player::new(String::from("Default"))
    }
}