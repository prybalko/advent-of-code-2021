use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Entry {
    Marked,
    Unmarked(usize)
}

const N: usize = 5;

type Board = [[Entry; N];N];


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input").unwrap();
    let buf_reader = BufReader::new(file);

    let mut reader = buf_reader.lines();
    let input = reader.next().unwrap().unwrap().split(',').map(|i| i.parse().unwrap()).collect::<Vec<usize>>();

    let mut boards: Vec<Board> = vec![];

    while let Some(Ok(_)) = reader.next() {
        let mut lines: Vec<[Entry; N]> = vec![];
        for _ in 0..N {
            let line = reader
                .next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| Entry::Unmarked(s.parse().unwrap())).collect::<[Entry; N]>();
            lines.push(line);
        }
        boards.push(lines as Board);
    }

    for i in input {
        lines.iter_mut()
            .flat_map(|e| e.iter_mut())
            .filter(|e| matches!(e, Entry::Unmarked(x) if *x == i))
            .for_each(|e| *e = Entry::Marked);

        for block in lines {
            for row in block {

            }
        }
    }

    println!("Ok");
    Ok(())
}
