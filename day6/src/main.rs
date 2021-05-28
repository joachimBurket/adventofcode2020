use std::fs;
use std::collections::HashMap;

fn main() {
    const ANSWERS_FILENAME: &str = "resources/answers";
    let input = fs::read_to_string(ANSWERS_FILENAME).unwrap();

    let groups: Vec<&str> = input.split("\n\n").collect();
    println!("There are {} groups of answers", groups.len());

    let mut yes_answers_count1 = 0;
    let mut yes_answers_count2 = 0;

    for group in groups {
        let group_members = group.lines().count();
        let group_answers = group.replace("\n", "");
        yes_answers_count1 += count_yes_answers_part1(&group_answers);
        yes_answers_count2 += count_yes_answers_part2(&group_answers, group_members);
    }

    println!(
        "Part1: There was a total of {} answers to which anyone in the group answered 'yes'",
        yes_answers_count1,
    );
    println!(
        "Part2: There was a total of {} answers to which everyone in the group answered 'yes'",
        yes_answers_count2,
    ) 
}

/// Takes the answers of a group and returns the number of questions that anyone in the group answered "yes"
fn count_yes_answers_part1(answers: &str) -> usize {
    let answers_map = get_answers_map(answers);
    answers_map.len()
}

/// Takes the answers of a group and returns the number of questions that everyone in the group answered "yes"
fn count_yes_answers_part2(answers: &str, group_members: usize) -> usize {
    let answers_map = get_answers_map(answers);
    
    answers_map.iter().fold(0, |acc, x| {
        if *x.1 == group_members { acc + 1 } else { acc }
    })
}

fn get_answers_map(answers: &str) -> HashMap<char, usize> {
    let mut answers_map: HashMap<char, usize> = HashMap::new();
    for letter in answers.chars() {
        if let Some(x) = answers_map.get_mut(&letter) {
            *x += 1;
        }
        else {
            answers_map.insert(letter, 1);
        }
    }
    answers_map
}