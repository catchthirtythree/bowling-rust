use rand::Rng;

const MAX_FRAMES_PER_GAME: i32 = 10;
const MAX_ROLL_SCORE: i32 = 10;

fn main() {
    // The rolls of a single player in a game
    let mut rolls: Vec<i32> = vec![];

    // Roll for 10 frames
    for _ in 1..=MAX_FRAMES_PER_GAME {
        // Get the current frame number so we can use it for applying
        // the proper rules to the frame (ie: the last frame + bonus rolls).
        let frame_number: i32 = get_current_frame_number(&rolls);

        // Roll for a frame and get the score(s)
        let mut frame_roll_scores: Vec<i32> = roll_frame(frame_number);

        // Display the frame stats.
        println!("Frame: {}, Rolls: {:?}",
            get_current_frame_number(&rolls), frame_roll_scores);

        // Append the frame roll score(s) to the list of rolls
        rolls.append(&mut frame_roll_scores);
    }
}

fn get_current_frame_number(rolls: &Vec<i32>) -> i32 {
    let mut frame_number: i32 = 1;
    let mut roll_number: i32 = 1;

    for roll in rolls {
        if *roll == MAX_ROLL_SCORE || roll_number == 2 {
            frame_number += 1;
            roll_number = 1;
        } else {
            roll_number += 1;
        }
    }

    frame_number
}

fn roll_ball(max: i32) -> i32 {
    // The min-max of a bowling roll score
    let roll_range = 0..=max;

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
        let third_roll = roll_ball(MAX_ROLL_SCORE - second_roll);

        rolls.push(second_roll);
        rolls.push(third_roll);

        rolls
    } else {
        // Roll the second ball and if the sum of the first two scores is the
        // max, then we should let the player roll one more time, otherwise,
        // they cannot roll anymore
        let second_roll = roll_ball(MAX_ROLL_SCORE - first_roll);

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
    rolls.push(roll_ball(MAX_ROLL_SCORE - first_roll));

    return rolls;
}
