use std::fs;
use day5::Plane;

fn main() {
    const BOARDINGPASS_FILENAME: &str = "resources/boardingpasses";
    
    let boardingpasses = fs::read_to_string(BOARDINGPASS_FILENAME).unwrap();
    let plane = Plane::new(&boardingpasses);
    
    // Part 1
    let highest_seat_id = plane.get_highest_seat_id();
    println!("The highest seat ID is the {}", highest_seat_id);    
    
    // Part 2
    let my_seat_id = plane.get_my_seat_id();
    println!("My seat is the {}", my_seat_id.unwrap());    
}