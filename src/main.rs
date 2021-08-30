use rand::Rng;
use std::collections::VecDeque;
use std::iter::FromIterator;

const MAX_FRAMES_PER_GAME: i32 = 10;
const MAX_ROLL_SCORE: i32 = 10;
const MIN_ROLL_SCORE: i32 = 0;

fn main() {
    // The rolls of a single player in a game
    let mut rolls: Vec<i32> = vec![];

    // Roll for 10 frames
    for frame_number in 1..=MAX_FRAMES_PER_GAME {
        // Roll for a frame and get the score(s)
        let frame_roll_scores: Vec<i32> = roll_frame(frame_number);

        // Append the frame roll score(s) to the list of rolls
        rolls.append(&mut frame_roll_scores.clone());

        // Get the current score for the player
        let current_score = calculate_score(&mut VecDeque::from_iter(rolls.clone()));

        // Display the frame stats
        println!("Frame: {:2}, Rolls: {:2?}, Score: {}",
            frame_number, frame_roll_scores, current_score);
    }
}

fn calculate_score(rolls: &mut VecDeque<i32>) -> i32 {
    let mut current_score: i32 = 0;

    for current_frame in 1..=MAX_FRAMES_PER_GAME {
        if current_frame == MAX_FRAMES_PER_GAME {
            current_score += calculate_score_last_frame(rolls);
        } else {
            current_score += calculate_score_regular_frame(rolls);
        }
    }

    current_score
}

fn calculate_score_last_frame(rolls: &mut VecDeque<i32>) -> i32 {
    let mut frame_score: i32 = 0;

    if let Some(roll1) = rolls.pop_front() {
        frame_score += roll1;

        if roll1 == MAX_ROLL_SCORE {
            if let Some(b1) = rolls.get(0) {
                frame_score += b1;
            }

            if let Some(b2) = rolls.get(1) {
                frame_score += b2;
            }
        } else {
            if let Some(roll2) = rolls.pop_front() {
                frame_score += roll2;

                if roll1 + roll2 == MAX_ROLL_SCORE {
                    if let Some(b1) = rolls.get(0) {
                        frame_score += b1;
                    }

                    if let Some(roll3) = rolls.get(0) {
                        frame_score += roll3;
                    }
                }
            }
        }
    }

    frame_score
}

fn calculate_score_regular_frame(rolls: &mut VecDeque<i32>) -> i32 {
    let mut frame_score: i32 = 0;

    if let Some(roll1) = rolls.pop_front() {
        frame_score += roll1;

        if roll1 == MAX_ROLL_SCORE {
            if let Some(b1) = rolls.get(0) {
                frame_score += b1;
            }

            if let Some(b2) = rolls.get(1) {
                frame_score += b2;
            }
        } else {
            if let Some(roll2) = rolls.pop_front() {
                frame_score += roll2;

                if roll1 + roll2 == MAX_ROLL_SCORE {
                    if let Some(b1) = rolls.get(0) {
                        frame_score += b1;
                    }
                }
            }
        }
    }

    frame_score
}

fn roll_ball(max: i32) -> i32 {
    // The min-max of a bowling roll score
    let roll_range = MIN_ROLL_SCORE..=max;

    // Generate random number in the range
    rand::thread_rng().gen_range(roll_range)
}

fn roll_frame(frame_number: i32) -> Vec<i32> {
    if frame_number > MAX_FRAMES_PER_GAME {
        panic!("{}", "Cannot bowl passed the final frame of the game.")
    }

    if frame_number == MAX_FRAMES_PER_GAME {
        roll_last_frame()
    } else {
        roll_regular_frame()
    }
}

fn roll_last_frame() -> Vec<i32> {
    // A list to keep trqack of the rolls for the frame
    let mut rolls: Vec<i32> = vec![];

    // Roll the first ball in the frame
    let first_roll: i32 = roll_ball(MAX_ROLL_SCORE);

    // Append the first roll to the list of rolls
    rolls.push(first_roll);

    // If the player's roll is the max, let them roll two bonus rolls
    if first_roll == MAX_ROLL_SCORE {
        // Roll the next two bonus balls and add them to the list of rolls
        let second_roll = roll_ball(MAX_ROLL_SCORE);
        let third_roll = roll_ball(MAX_ROLL_SCORE - second_roll + MIN_ROLL_SCORE);

        rolls.push(second_roll);
        rolls.push(third_roll);

        rolls
    } else {
        // Roll the second ball and if the sum of the first two scores is the
        // max, then we should let the player roll one more time, otherwise,
        // they cannot roll anymore
        let second_roll = roll_ball(MAX_ROLL_SCORE - first_roll + MIN_ROLL_SCORE);

        rolls.push(second_roll);

        if first_roll + second_roll == MAX_ROLL_SCORE {
            let third_roll = roll_ball(MAX_ROLL_SCORE);

            rolls.push(third_roll);
        }

        rolls
    }
}

fn roll_regular_frame() -> Vec<i32> {
    // A list to keep track of the rolls for the frame
    let mut rolls: Vec<i32> = vec![];

    // Roll the first ball in the frame
    let first_roll = roll_ball(MAX_ROLL_SCORE);

    // Append the first roll to the list of rolls
    rolls.push(first_roll);

    // The first roll is the max, then we don't have to roll again
    if first_roll == MAX_ROLL_SCORE {
        return rolls;
    }

    // Append the second roll to the list
    rolls.push(roll_ball(MAX_ROLL_SCORE - first_roll + MIN_ROLL_SCORE));

    return rolls;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_score_120_game() {
        let rolls = [10, 7, 3, 2, 1, 7, 3, 4, 6, 2, 6, 0, 10, 8, 0, 4, 1, 10, 9, 1];

        assert_eq!(calculate_score(&mut VecDeque::from_iter(rolls)), 120);
    }

    #[test]
    fn test_calculate_score_perfect_game() {
        let rolls = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];

        assert_eq!(calculate_score(&mut VecDeque::from_iter(rolls)), 300);
    }
}