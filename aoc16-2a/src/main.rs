// Advent of Code 2016, round 2a

use std::io;

// Get a keypad move
// @param current_key The number button you're currently on
// @param direction The direction to move in.
// @return The new number button after current move
fn keypad_move_one(current_key: u8, direction: char) -> Option<u8> {
    return match (current_key, direction) {
        // TODO Refactor to an array? It won't save space, though...
        (1, 'R') => Some(2),
        (1, 'U') => Some(1),
        (1, 'L') => Some(1),
        (1, 'D') => Some(4),

        (2, 'R') => Some(3),
        (2, 'U') => Some(2),
        (2, 'L') => Some(1),
        (2, 'D') => Some(5),

        (3, 'R') => Some(3),
        (3, 'U') => Some(3),
        (3, 'L') => Some(2),
        (3, 'D') => Some(6),

        (4, 'R') => Some(5),
        (4, 'U') => Some(1),
        (4, 'L') => Some(4),
        (4, 'D') => Some(7),

        (5, 'R') => Some(6),
        (5, 'U') => Some(2),
        (5, 'L') => Some(4),
        (5, 'D') => Some(8),

        (6, 'R') => Some(6),
        (6, 'U') => Some(3),
        (6, 'L') => Some(5),
        (6, 'D') => Some(9),

        (7, 'R') => Some(8),
        (7, 'U') => Some(4),
        (7, 'L') => Some(7),
        (7, 'D') => Some(7),

        (8, 'R') => Some(9),
        (8, 'U') => Some(5),
        (8, 'L') => Some(7),
        (8, 'D') => Some(8),

        (9, 'R') => Some(9),
        (9, 'U') => Some(6),
        (9, 'L') => Some(8),
        (9, 'D') => Some(9),

        (_, _)   => Some(current_key),

    }
}


fn keypad_move_line(mut current_key: u8, direction: &String) -> Option<u8> {
    for c in direction.chars() {
        match keypad_move_one(current_key, c) {
            Some(new_number) => current_key = new_number,
            None             => return None
        }
    }
    Some(current_key)
}

fn main() {
    // The key we're currently on
    let mut current_key: u8 = 5;

    print!("Key sequence: ");
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => current_key = keypad_move_line(current_key, &line).unwrap(),
            Err(_) => break
        }
        print!("{}", current_key);
    }
    println!("");
}
