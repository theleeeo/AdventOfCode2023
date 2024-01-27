use std::fmt::{self, Display, Formatter};

pub struct Map {
    map: Vec<Vec<Option<Pipe>>>,
    pub start: (usize, usize),
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.map {
            for tile in row {
                match tile {
                    Some(tile) => write!(f, "{tile}")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn get(&self, x: usize, y: usize) -> Option<&Pipe> {
        self.map
            .get(y)
            .and_then(|row| row.get(x))
            .and_then(|tile| tile.as_ref())
    }

    pub fn get_first_directions(&self) -> Vec<Direction> {
        let (x, y) = self.start;
        let mut directions = Vec::new();
        if y > 0 {
            if let Some(tile) = self.get(x, y - 1) {
                if tile.can_connect(&Direction::South) {
                    directions.push(Direction::North);
                }
            }
        }
        if let Some(tile) = self.get(x + 1, y) {
            if tile.can_connect(&Direction::West) {
                directions.push(Direction::East);
            }
        }
        if let Some(tile) = self.get(x, y + 1) {
            if tile.can_connect(&Direction::North) {
                directions.push(Direction::South);
            }
        }
        if x > 0 {
            if let Some(tile) = self.get(x - 1, y) {
                if tile.can_connect(&Direction::East) {
                    directions.push(Direction::West);
                }
            }
        }
        directions
    }

    pub fn step(&self, (x, y): (usize, usize), direction: &Direction) -> (usize, usize) {
        let (dx, dy) = direction.offset();
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        (nx as usize, ny as usize)
    }
}

pub fn parse_map(input: &str) -> Map {
    let map_vec: Vec<Vec<Option<Pipe>>> = input
        .lines()
        .map(|line| line.chars().map(parse_tile).collect())
        .collect();

    let mut start = (0, 0);
    'find_start: for (y, row) in map_vec.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(Pipe::Start) = tile {
                start = (x, y);
                break 'find_start;
            }
        }
    }

    Map {
        map: map_vec,
        start,
    }
}

fn parse_tile(input: char) -> Option<Pipe> {
    match input {
        '-' => Some(Pipe::Horizontal),
        '|' => Some(Pipe::Vertical),
        'L' => Some(Pipe::NorthEast),
        'F' => Some(Pipe::SouthEast),
        '7' => Some(Pipe::SouthWest),
        'J' => Some(Pipe::NorthWest),
        'S' => Some(Pipe::Start),
        '.' => None,
        a => panic!("Invalid tile: {a}"),
    }
}

pub enum Pipe {
    Horizontal,
    Vertical,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
    Start,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pipe::Horizontal => '-',
                Pipe::Vertical => '|',
                Pipe::NorthEast => 'L',
                Pipe::SouthEast => 'F',
                Pipe::SouthWest => '7',
                Pipe::NorthWest => 'J',
                Pipe::Start => 'S',
            }
        )
    }
}

impl Pipe {
    // If entering the pipe from the given direction, what direction will you exit?
    pub fn out_direction(&self, in_direction: &Direction) -> Option<Direction> {
        match (self, in_direction) {
            (Pipe::Horizontal, Direction::East) => Some(Direction::East),
            (Pipe::Horizontal, Direction::West) => Some(Direction::West),
            (Pipe::Vertical, Direction::North) => Some(Direction::North),
            (Pipe::Vertical, Direction::South) => Some(Direction::South),
            (Pipe::NorthEast, Direction::South) => Some(Direction::East),
            (Pipe::NorthEast, Direction::West) => Some(Direction::North),
            (Pipe::SouthEast, Direction::North) => Some(Direction::East),
            (Pipe::SouthEast, Direction::West) => Some(Direction::South),
            (Pipe::SouthWest, Direction::North) => Some(Direction::West),
            (Pipe::SouthWest, Direction::East) => Some(Direction::South),
            (Pipe::NorthWest, Direction::South) => Some(Direction::West),
            (Pipe::NorthWest, Direction::East) => Some(Direction::North),
            _ => None,
        }
    }

    // Can the pipe connect to the given direction?
    pub fn can_connect(&self, direction: &Direction) -> bool {
        matches!(
            (self, direction),
            (Pipe::Horizontal, Direction::East)
                | (Pipe::Horizontal, Direction::West)
                | (Pipe::Vertical, Direction::North)
                | (Pipe::Vertical, Direction::South)
                | (Pipe::NorthEast, Direction::North)
                | (Pipe::NorthEast, Direction::East)
                | (Pipe::SouthEast, Direction::South)
                | (Pipe::SouthEast, Direction::East)
                | (Pipe::SouthWest, Direction::South)
                | (Pipe::SouthWest, Direction::West)
                | (Pipe::NorthWest, Direction::North)
                | (Pipe::NorthWest, Direction::West)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}
