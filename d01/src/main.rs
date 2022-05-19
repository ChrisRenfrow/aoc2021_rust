use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// TDD baybee

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_basic() {
        let test_data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(depth_increments(test_data), 7);
    }

    #[test]
    fn b_no_increment() {
        let test_data = vec![100, 75, 50, 25, 0];
        assert_eq!(depth_increments(test_data), 0);
    }

    #[test]
    fn c_all_equal() {
        let test_data = vec![42, 42, 42];
        assert_eq!(depth_increments(test_data), 0);
    }
}

/////////////

fn main() {
    let input = read_input("./input.txt");
    println!("{}", depth_increments(input));
}

fn depth_increments(readings: Vec<i32>) -> i32 {
    let mut curr = readings[0];
    let mut inc = 0;
    for d in readings {
        if d > curr {
            inc += 1;
        }
        curr = d;
    }
    inc
}

fn parse_num(string: String) -> i32 {
    string.parse().unwrap()
}

fn read_input(path: &str) -> Vec<i32> {
    let f = File::open(path).expect("Error opening file");
    let f = BufReader::new(f);
    let mut input: Vec<i32> = Vec::new();

    for l in f.lines() {
        input.push(parse_num(l.unwrap()));
    }
    input
}
