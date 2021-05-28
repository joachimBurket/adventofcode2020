pub struct Plane {
    seats: Vec<Seat>,
}

impl Plane {
    const MAX_SEATS: u32 = 128 * 8;

    pub fn new(boardingpasses: &str) -> Plane {
        Self {
            seats: Plane::parse_boardingpasses(boardingpasses),
        }
    }

    fn parse_boardingpasses(input: &str) -> Vec<Seat> {
        let mut seats: Vec<Seat> = Vec::new();
    
        for line in input.lines() {
            seats.push(Seat::new(line));
        }
    
        seats
    }

    pub fn get_highest_seat_id(&self) -> u32 {
        let mut highest_id: u32 = 0;
        for seat in &self.seats {
            if seat.id > highest_id {
                highest_id = seat.id;
            }
        }
        highest_id
    }

    pub fn get_my_seat_id(&self) -> Option<u32> {
        let mut my_seat_id = None;
        for seat_id in 1..Plane::MAX_SEATS-1 {
            if !self.seat_occupied(seat_id) {
                // empty seat
                println!("seat {} is empty", seat_id);
                
                // checks if id -1 and +1 are present
                if self.seat_occupied(seat_id-1) && self.seat_occupied(seat_id+1) {
                    println!("Thats my seat!");
                    my_seat_id = Some(seat_id);
                    break;
                }
            }
        }
        my_seat_id
    }

    pub fn seat_occupied(&self, seat_id: u32) -> bool {
        self.seats.iter().any(|s| s.id == seat_id)
    }
}


struct Seat {
    pos: (u32, u32),   // (row, col)
    id: u32,
}

impl Seat {
    pub fn new(input: &str) -> Seat {
        let pos = Seat::get_pos(input);
        let id = Seat::get_id(pos);
        // println!("Boarding pass {} has pos {:?} and id {}", input, pos, id);
        
        Self {
            pos,
            id,
        }
    }

    fn get_pos(input: &str) -> (u32, u32) {
        let row_chars = input[0..7]
            .replace("F", "0")
            .replace("B", "1");
        let row = u32::from_str_radix(&row_chars, 2).unwrap();

        let col_chars = input[7..10]
            .replace("L", "0")
            .replace("R", "1");
        let col = u32::from_str_radix(&col_chars, 2).unwrap();

        (row, col)
    }

    fn get_id(pos: (u32, u32)) -> u32 {
        pos.0 * 8 + pos.1
    }
}