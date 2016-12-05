// Advent of Code 2016 round 1a
// Usage: aoc16-1a < input.txt

extern crate num;

use num::complex::Complex;
use std::io;
use std::io::Read;
use std::option::Option;
use std::string::String;


fn find_duplicate_coordinates(directions: &std::string::String) -> Option<Complex<i64>> {
    // Current coordinates
    let mut coords = Complex::<i64>::new(0, 0);
    // Direction we're traveling in, default north
    let mut direction = Complex::<i64>::new(0, 1);
    let mut distance: i64 = 0;

    // Addition in 1b: holds the visited locations
    let mut visited_locations = vec![coords];

    for c in directions.chars() {
        match c {
            // Turn ~ rotate by i
            'L' => direction = direction * Complex::new(0, 1),
            'R' => direction = direction * Complex::new(0, -1),
            // Accumulate distance
            '0' ... '9' => distance = distance * 10 + (c as i64 - '0' as i64),
            ',' => {
                for _ in 0..distance {
                    coords = coords + direction;
                    // println!("New coordinates: {}, {}", coords.re, coords.im);
                    if visited_locations.contains(&coords) {
                        return Some(coords);
                    }
                    visited_locations.push(coords);
                }
                distance = 0;
            },
            _ => { }
        }
    }

    None
}


fn main() {

    let mut directions = String::new();
    io::stdin()
        .read_to_string(&mut directions)
        .expect("Could not read stdin!");

    match find_duplicate_coordinates(&directions) {
        Some(coordinates) => println!("First duplicate coordinates are at distance {}",
            coordinates.re.abs() + coordinates.im.abs()),
        None => println!("No location was visited twice.")
    }

}
