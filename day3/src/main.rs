use std::fs;
use array2d::Array2D;
use vector2d::Vector2D;

struct Toboggan {
    map: Array2D<char>,
    tree_char: char, 
    slope_dir: Vector2D<usize>,
}

impl Toboggan {
    fn new(content: &String, tree_char: char, slope_dir: Vector2D<usize>) -> Self {
        let lines: Vec<&str> = content.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        
        let chars: Vec<char> = lines.iter()
        .map(|s| s.chars())
        .flatten()
        .collect();

        Self {
            map: Array2D::from_row_major(&chars, height, width),
            tree_char,
            slope_dir,
        }
    }

    fn slope(&self) -> u32 {
        let mut trees: u32 = 0;
        let mut current_pos = Vector2D::new(0,0);
        
        while current_pos.y < self.map.num_rows() {
            let elt = *self.map.get(current_pos.y, current_pos.x).unwrap();
            if elt == self.tree_char {
                trees += 1;
            }
            current_pos = self.update_pos(current_pos.clone());
        }
        return trees;
    }

    fn update_pos(&self, mut pos: Vector2D<usize>) -> Vector2D<usize> {
        pos += self.slope_dir;
        if pos.x >= self.map.num_columns() {
            pos.x = pos.x - self.map.num_columns();
        }
        return pos;
    }
}


fn main() {
    const TOBOGGAN_FILENAME: &str = "resources/toboggan";
    let content = fs::read_to_string(TOBOGGAN_FILENAME).unwrap();

    let slope_dirs = [
        Vector2D::new(1,1),
        Vector2D::new(3,1),
        Vector2D::new(5,1),
        Vector2D::new(7,1),
        Vector2D::new(1,2),
    ];
    let mut trees: Vec<u32> = Vec::new();

    for slope_dir in slope_dirs.iter() {
        let toboggan = Toboggan::new(&content, '#', *slope_dir);
        trees.push(toboggan.slope());
        println!("Encountered {} trees", trees.last().unwrap());
    }
    
    let result: u64 = trees.iter().fold(1, |acc: u64, x: &u32| acc * (*x as u64));
    println!("Final result = {}", result);
}
