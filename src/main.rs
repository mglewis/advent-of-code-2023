extern crate argparse;

use advent_of_code_2023::read_file;
use argparse::{ArgumentParser, Store};
use days::*;
mod days;
use std::fmt;

#[derive(Debug)]
enum ReturnType {
    UInt32(u32),
    Int64(i64),
    String(String),
}

impl From<u32> for ReturnType {
    fn from(u: u32) -> Self {
        Self::UInt32(u)
    }
}

impl From<i64> for ReturnType {
    fn from(i: i64) -> Self {
        Self::Int64(i)
    }
}

impl From<String> for ReturnType {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReturnType::UInt32(value) => write!(f, "{}", value),
            ReturnType::Int64(value) => write!(f, "{}", value),
            ReturnType::String(value) => write!(f, "{}", value),
        }
    }
}

fn main() {
    let mut day = 0;
    let mut part = ' ';
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Run Advent of Code 2022.");
        ap.refer(&mut day)
            .add_option(&["-d", "--day"], Store, "Day to run");
        ap.refer(&mut part)
            .add_option(&["-p", "--part"], Store, "Part to run [a, b]");
        ap.parse_args_or_exit();
    }

    if day < 1 || day > 25 {
        panic!("--day parameter must be specified and be between `1` and `25` inclusive");
    }

    let valid_parts = vec!['a', 'b'];
    if !valid_parts.contains(&part) {
        panic!("--part parameter must be specified and be set either `a` or `b`");
    }

    let input = read_file(day);

    let result: ReturnType = match (day, part) {
        (1, 'a') => day_1::part_a(&input).into(),
        (_, _) => panic!("Unrecognised day [{}] part [{}]", day, part),
    };

    println!("Result: {}", result)
}
