use std::fs;


struct Bag {
    color: String,
    content: Vec<Bag>,
}

impl Bag {
    fn new(input: &str) -> Self {
        let content: Vec<Bag> = Vec::new();

        let splitted: Vec<&str> = input.split("contain").collect();
        let color: String = splitted[0].replace(" bags ", "");
        let content = splitted[1];
        
        if 

        Self {
            color,
            content,
        }
    }
}

fn main() {
    const BAGS_FILENAME: &str = "resources/tests";
    let input = fs::read_to_string(BAGS_FILENAME).unwrap();
    let bags = parse_bags(&input);
}

fn parse_bags(input: &str) -> Vec<Bag> {
    let mut bags: Vec<Bag> = Vec::new();
    
    for line in input.lines() {
        bags.push(Bag::new(line));
    }
    bags
}


fn can_contain_bag(bag: &Bag) -> Vec<Bag> {
    let mut can_contain: Vec<Bag> = Vec::new();
    can_contain
}
