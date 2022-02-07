use std::fs::File;
use std::io::{BufRead, BufReader};

fn power_consumption(lines: &Vec<Vec<u8>>) -> usize {
    let columns = lines[0].len();
    let mut rows = lines.len();
    let mut ones:Vec<usize> = vec![0;columns];

    for i in 0..columns {
        for line in 0..rows {
            ones[i] += lines[line][i] as usize;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in ones {
        gamma = (gamma << 1) + (i*2 > rows) as usize;
        epsilon = (epsilon << 1) + (i*2 < rows) as usize;
    }

    gamma * epsilon
}

fn co2(lines: &Vec<Vec<u8>>) -> usize {

}

fn parse_line(line: String) -> Vec<u8> {
    line.chars().map(|c| String::from(c).parse().unwrap()).collect()
}

fn main() {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);
    let lines: Vec<Vec<u8>> = buf_reader
        .lines()
        .flat_map(|line|
            line.map(|line|
                parse_line(line)
            ))
        .collect();

    println!("Power consumption: {}", power_consumption(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parser() {
        let line = "101";
        assert_eq!(parse_line(line.to_string()), vec![1,0,1]);
    }

    #[test]
    fn test_power_consumption() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010".to_string();
        let lines = input.split("\n").map(|line| parse_line(line.to_string())).collect();
        let power = power_consumption(&lines);
        assert_eq!(power, 198);

    }
}