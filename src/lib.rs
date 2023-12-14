use std::{env, fs};

pub fn to_u32(i: &str) -> u32 {
    i.trim().parse::<u32>().unwrap()
}

pub fn to_u64(i: &str) -> u64 {
    i.trim().parse::<u64>().unwrap()
}

pub fn to_i64(i: &str) -> i64 {
    i.trim().parse::<i64>().unwrap()
}

pub fn read_file(day: u32) -> String {
    read_file_path(day, vec!["inputs"])
}

pub fn read_test_file(day: u32) -> String {
    read_file_path(day, vec!["inputs", "test"])
}

fn read_file_path(day: u32, path: Vec<&str>) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = format!("day_{}.txt", day);
    let filepath = path.iter().fold(cwd, |c, x| c.join(x));
    let f = fs::read_to_string(filepath.join(filename.clone()));
    f.expect(&format!("could not open input file {}", filename))
}
