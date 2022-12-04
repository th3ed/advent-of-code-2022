// A | X - Rock (1pt)
// B | Y - Paper (2pt)
// C | Z - Scissors (3pt)
//
// Lose - 0pt
// Draw - 3pt
// Win - 6pt
fn remap(choice: char) -> char {
    match choice {
        'X' => 'A',
        'Y' => 'B',
        _ => 'C'
    }
}

fn choice_points(choice: char) -> i32 {
    match choice {
        'A' => 1,
        'B' => 2,
        _ => 3
    }
}

fn beats(choice: char) -> char {
    match choice {
        'A' => 'C',
        'B' => 'A',
        _ => 'B'
    }
}

fn score(opponent: char, player: char) -> i32 {
    // Map the player char to the same as opponent
    let player_ = remap(player);

    // Check if we won or lost
    let beats_ = beats(opponent);

    let score_beat: i32 = match player_ {
        _ if player_ == beats_ => 0,
        _ if player_ == opponent => 3,
        _ => 6
    };

    let score_choice: i32 = choice_points(player_);
    score_beat + score_choice
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let o = chars.next().unwrap();
            _ = chars.next();
            let p = chars.next().unwrap();
            score(o, p)
        }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}
