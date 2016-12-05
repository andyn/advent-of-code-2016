// Advent of Code 2016 round 1a
// Usage: aoc16-1a < input.txt

extern crate num;

use num::complex::Complex;
use std::io;
use std::io::Read;

fn main() {

    // Current coordinates
    let mut coords = Complex::<i64>::new(0, 0);
    // Direction we're traveling in, default north
    let mut direction = Complex::<i64>::new(0, 1);
    let mut distance: i64 = 0;

    let mut directions = std::string::String::new();
    io::stdin()
        .read_to_string(&mut directions)
        .expect("Could not read stdin!");
    for c in directions.chars() {
        match c {
            'L' => direction = direction * Complex::new(0, 1),
            'R' => direction = direction * Complex::new(0, -1),
            '0' ... '9' => distance = distance * 10 + (c as i64 - '0' as i64),
            ',' => {
                coords = coords + direction * distance;
                distance = 0;
                // println!("New coordinates: {}, {}", coords.re, coords.im);
            }
            _ => { }
        }
    }

    coords = coords + direction * distance;
    println!("Distance: {}", coords.re.abs() + coords.im.abs());
}
