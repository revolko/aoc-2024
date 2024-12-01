use std::{cmp::Reverse, collections::BinaryHeap};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for l in input.lines() {
        let mut raw_locations = l.split_whitespace();
        left.push(raw_locations.next().unwrap().parse::<u32>().unwrap());
        right.push(raw_locations.next().unwrap().parse::<u32>().unwrap());
    }

    return (left, right);
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (left, right) = input;
    let mut sum: u32 = 0;

    let mut left_heap = BinaryHeap::from_iter(left.iter().map(|&i| Reverse(i)));
    let mut right_heap = BinaryHeap::from_iter(right.iter().map(|&i| Reverse(i)));

    for _ in 0..left.len() {
        sum += left_heap
            .pop()
            .unwrap()
            .0
            .abs_diff(right_heap.pop().unwrap().0);
    }

    return sum;
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (left, right) = input;
    let mut sim_sum = 0;
    for l in left {
        let mut count = 0;
        for r in right {
            if l == r {
                count += 1;
            }
        }

        sim_sum += l * count;
    }
    return sim_sum;
}
