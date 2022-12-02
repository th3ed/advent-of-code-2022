use std::collections::BinaryHeap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let mut elves = Vec::new();
    let mut stack: Vec<i32> = Vec::new();
    for line in input.lines() {
        if line == "" {
            // End of food for given elf
            elves.push(stack.iter().sum());
            stack.clear()
        } else {
            // Next value, add to stack
            stack.push(line.parse::<i32>().unwrap())
        }
    }

    elves
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    *input.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    input.select_nth_unstable(index)
}

