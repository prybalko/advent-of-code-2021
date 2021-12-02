use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn number_of_depth_increases(depth: &[usize]) -> usize{
    depth.windows(2).fold(0, |acc, window| {
        if window[1] > window[0] {
            acc + 1
        } else {
            acc
        }
    })
}


fn sums(depth: &[usize]) -> Vec<usize>{
    depth.windows(3).map(|window| window.iter().sum()).collect()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);

    let lines: Vec<usize> = buf_reader.lines().map(|line|  line.map(|line| line.parse::<usize>().unwrap()) ).flatten().collect();

    println!("{}", number_of_depth_increases(&lines));
    println!("{}", number_of_depth_increases(&sums(&lines)));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_a() {
        let test_data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(number_of_depth_increases(&test_data), 7);
    }

    #[test]
    fn example_part_b() {
        let test_data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let sums = sums(&test_data);
        assert_eq!(sums[0], 607);
        assert_eq!(sums[1], 618);
        assert_eq!(number_of_depth_increases(&sums), 5);
    }
}
