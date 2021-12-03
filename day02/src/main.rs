use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

enum Command {
    Forward(isize),
    Down(isize),
    Up(isize)
}

#[derive(Debug, PartialEq)]
struct Vector {
    x: isize,
    y: isize
}

impl Vector {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl From<&Command> for Vector {
    fn from(command: &Command) -> Self {
        match command {
            Command::Up(x) => Vector::new(0, -(*x)),
            Command::Down(x) => Vector::new(0, *x),
            Command::Forward(x) => Vector::new(*x, 0)
        }
    }
}

#[derive(Debug, Clone)]
struct ParseLineError {
    cause: String,
}

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse line: {}", self.cause)
    }
}

impl ParseLineError {
    fn new(cause: String) -> Self {
        Self { cause }
    }
}

impl Error for ParseLineError {}

impl TryFrom<String> for Command {
    type Error = ParseLineError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split(" ");
        let command = split.next().ok_or_else(|| Self::Error::new("No split point found".to_string()))?;
        let v: isize = split
            .next()
            .ok_or_else(|| Self::Error::new("No second split".to_string()))?
            .parse()
            .map_err(|err| Self::Error::new(format!("Distance is not a number: {}", err)))?;
        
        match command {
            "forward" => Ok(Command::Forward(v)),
            "up" => Ok(Command::Up(v)),
            "down" => Ok(Command::Down(v)),
            _ => Err(Self::Error::new("Unknown command".to_string()))
        }

    }
}

fn track(commands: &[Command]) -> Vector {
    commands.iter().fold(Vector::new(0, 0), |acc, p| acc + p.into() )
}

fn aim(commands: &[Command]) -> Vector {
    let mut aim = 0;
    let mut position = Vector::new(0,0);
    for command in commands {
        match command {
            Command::Up(x) => {aim -= x},
            Command::Down(x) => {aim += x}
            Command::Forward(x) => {
                position.x += x;
                position.y += aim*x;
            }
        }
    }
    position
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);

    let commands = buf_reader
        .lines()
        .map(|line| line.map(|line| line.try_into()))
        // .map(|line| line.map(|line| Command::try_from(line)))
        .flatten()
        .collect::<Result<Vec<Command>, _>>()?;

    let position = track(&commands);

    println!("{}", position.x * position.y);

    let position = aim(&commands);
    println!("{}", position.x * position.y);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let v = Vector::new(0, 1) + Vector::new(0, -1);
        assert_eq!(v, Vector::new(0, 0));
    }

    #[test]
    fn test_line_parser() {
        let parsed = Command::try_from("forward 5".to_string()).unwrap();
        assert!(matches!(parsed, Command::Forward(5)));
    }

    #[test]
    fn test_example() {
        let example = [
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        let position = track(&example);
        assert_eq!(position.x * position.y, 150);
    }

    #[test]
    fn test_aim() {
        let example = [
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        let position = aim(&example);
        assert_eq!(position.x * position.y, 900);
    }
}