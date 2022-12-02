use std::collections::BinaryHeap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> BinaryHeap<i32> {
    let mut elves: BinaryHeap<i32> = BinaryHeap::new();
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
pub fn solve_part1(input: &BinaryHeap<i32>) -> i32 {
    *input.peek().unwrap()
    // *input.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &BinaryHeap<i32>) -> i32 {
    let mut heap = input.clone();
    let mut out = 0;
    for _ in 0..3 {
        out += heap.pop().unwrap();
    }
    out
}
