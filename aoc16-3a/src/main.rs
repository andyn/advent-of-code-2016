// Advent of code, round 3

use std::io;

fn is_valid_triangle(triangle: &String) -> bool {
    let sides: Vec<&str> = triangle.split_whitespace().collect();
    if sides.len() != 3 {
        println!("bug");
        return false;
    }
    let side_lengths: Vec<i64> = sides
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let a: i64 = *side_lengths.get(0).unwrap();
    let b: i64 = *side_lengths.get(1).unwrap();
    let c: i64 = *side_lengths.get(2).unwrap();

    if a >= b + c { return false };
    if b >= a + c { return false };
    if c >= a + b { return false };
    true
}

fn main() {

    let mut correct_triangles = 0;
    let mut all_triangles = 0;
    let mut line = String::new();
    loop {
        match io::stdin().read_line(&mut line) {
            Err(_) | Ok(0) => break,
            Ok(_) => {
                if is_valid_triangle(&line) {
                    correct_triangles += 1;
                }
            }
        }
        all_triangles += 1;
        line = String::new();
    }
    println!("{} out of {} triangles are valid.",
        correct_triangles, all_triangles)
}
