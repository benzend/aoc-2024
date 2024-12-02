use std::fs;
use std::collections::HashMap;

pub fn run() {
    part_one();
    part_two();
}

fn part_one() {
    let data = data();
    let lines = data.lines();

    let mut lefts = Vec::new();
    let mut rights = Vec::new();

    for line in lines {
        let mut split = line.split_whitespace();
        let left = split.next().unwrap().parse::<usize>().unwrap();
        let right = split.next().unwrap().parse::<usize>().unwrap();
        lefts.push(left);
        rights.push(right);
    }

    lefts.sort_by(|a, b| a.cmp(b));
    rights.sort_by(|a, b| a.cmp(b));

    let mut total_dist = 0;

    for (i, left) in lefts.into_iter().enumerate() {
        let right = rights[i];
        let diff = if right > left {
            right - left
        } else {
            left - right
        };
        total_dist += diff;
    }

    println!("part one:");
    println!("{total_dist}");
}

fn part_two() {
    let data = data();
    let lines = data.lines();

    let mut lefts = Vec::new();
    let mut rights = Vec::new();

    for line in lines {
        let mut split = line.split_whitespace();
        let left = split.next().unwrap().parse::<usize>().unwrap();
        let right = split.next().unwrap().parse::<usize>().unwrap();
        lefts.push(left);
        rights.push(right);
    }

    let mut left_counts = HashMap::new();

    for left in lefts.iter() {
        let orig = left_counts.get(left.to_string().as_str());
        left_counts.insert(left.to_string(), orig.unwrap_or(&0) + 1);
    }

    let mut right_counts = HashMap::new();

    for right in rights.iter() {
        let orig = right_counts.get(right.to_string().as_str());
        right_counts.insert(right.to_string(), orig.unwrap_or(&0) + 1);
    }

    let mut total_score = 0;
    for left in lefts.iter() {
        let left_score = right_counts.get(left.to_string().as_str());

        let left_weighted = left * left_score.unwrap_or(&0);

        total_score += left_weighted;
    }

    println!("part two");
    println!("{}", total_score);
}

fn data() ->String {
    let file_path = "./data/day-1.txt";
    fs::read_to_string(file_path).expect("should have been able to read the file")
}
