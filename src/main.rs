extern crate quickhull;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() {
    let mut input: Vec<quickhull::Point> = Vec::new();

    // Get input from file
    let f = File::open("static/bird_points.dat").expect("File not found");

    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let mut words: Vec<&str> = l.split(' ').collect();
        words.retain(|&s| !(s == ""));
        if !(words.len() == 2) {
            println!("Not valid input: {:?}", words);
            continue;
        }
        let x = words[0].to_string().parse::<f64>().unwrap();
        let y = words[1].to_string().parse::<f64>().unwrap();
        input.push(quickhull::Point{x, y})
    }

    // Run algorithm
    let mut result: Vec<usize> = Vec::new();
    quickhull::run(&input, &mut result);

    // Print result
    for idx in result {
        println!("{:?}", input[idx]);
    }

}
