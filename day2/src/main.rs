use std::fs;
use regex::Regex;

struct Policy {
    appearing_times: [usize; 2],
    letter: char,
}

impl Policy {
    pub fn from_str(s: &str) -> Option<Self> {
        let res = Policy::regex().captures(s)?;

        Some(Policy {
            appearing_times: [res[1].parse().unwrap(), res[2].parse().unwrap()], 
            letter: res[3].chars().next()?,
        })
    }

    pub fn regex() -> Regex {
        Regex::new(r"^(\d+)-(\d+) (\w)$").unwrap()
    }

    // "1-3 a", "adeeissocas" => true
    pub fn check1(&self, content: &str) -> bool {
        let count = content
            .chars()
            .filter(|c| *c == self.letter)
            .count();
        (count >= self.appearing_times[0]) && (count <= self.appearing_times[1])
    }

    // "1-3 a", "adeeissocas" => true
    // "1-3 a", "adaeissocas" => false
    pub fn check2(&self, content: &str) -> bool {
        let c1 = content.chars().nth(self.appearing_times[0]).unwrap();
        let c2 = content.chars().nth(self.appearing_times[1]).unwrap();
        (c1 == self.letter) ^ (c2 == self.letter)
    }
}


fn parse_line(line: &str) -> (Policy, &str) {
    let v: Vec<&str> = line.split(':').collect();
    let policy = Policy::from_str(v[0]).unwrap();     
    let password: &str = v[1];

    (policy, password)
}

fn main() {
    const PASSWORDS_FILENAME: &str = "resources/passwords";

    let content = fs::read_to_string(PASSWORDS_FILENAME).unwrap();
    let mut passwords_valid1 : i32 = 0;
    let mut passwords_valid2 : i32 = 0;
    
    for line in content.lines() {
        let (policy, password) = parse_line(line);
        if policy.check1(password) {
            passwords_valid1 += 1;
        }
        if policy.check2(password) {
            passwords_valid2 += 1;
        }
    }
    println!("Part1: {} valid passwords", passwords_valid1);
    println!("Part2: {} valid passwords", passwords_valid2);
}
