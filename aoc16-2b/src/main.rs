// Advent of Code 2016, round 2a

use std::io;

// Get a keypad move
// @param current_key The number button you're currently on
// @param direction The direction to move in.
// @return The new number button after current move
//
// The keypad looks like this:
//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D
fn keypad_move_one(current_key: char, direction: char) -> char {
    return match (current_key, direction) {
        // TODO Refactor to an array? It won't save space, though...
        ('1', 'D') => '3',
        ('1', _)   => '1',

        ('2', 'R') => '3',
        ('2', 'D') => '6',
        ('2', _)   => '2',

        ('3', 'R') => '4',
        ('3', 'U') => '1',
        ('3', 'L') => '2',
        ('3', 'D') => '7',

        ('4', 'L') => '3',
        ('4', 'D') => '8',
        ('4', _)   => '4',

        ('5', 'R') => '6',
        ('5', _)   => '5',

        ('6', 'R') => '7',
        ('6', 'U') => '2',
        ('6', 'L') => '5',
        ('6', 'D') => 'A',

        ('7', 'R') => '8',
        ('7', 'U') => '3',
        ('7', 'L') => '6',
        ('7', 'D') => 'B',

        ('8', 'R') => '9',
        ('8', 'U') => '4',
        ('8', 'L') => '7',
        ('8', 'D') => 'C',

        ('9', 'L') => '8',
        ('9', _)   => '9',

        ('A', 'R') => 'B',
        ('A', 'U') => '6',
        ('A', _)   => 'A',

        ('B', 'R') => 'C',
        ('B', 'U') => '7',
        ('B', 'L') => 'A',
        ('B', 'D') => 'D',

        ('C', 'U') => '8',
        ('C', 'L') => 'B',
        ('C', _)   => 'C',

        ('D', 'U') => 'B',
        ('D', _)   => 'D',

        (_, _)     => current_key,
    }
}

// Check a single keypad move line
// @param char The current key you're on
// @param directions Line consisting of characters 'RULD'
// @return The new key after this line
fn keypad_move_line(mut current_key: char, directions: &String) -> char {
    for c in directions.chars() {
        current_key = keypad_move_one(current_key, c);
    }
    current_key
}


fn main() {
    let mut current_key = '5';

    print!("Key sequence: ");
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => current_key = keypad_move_line(current_key, &line),
            Err(_) => break
        }
        print!("{}", current_key);
    }
    println!("");
}
