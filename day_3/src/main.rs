const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut schematic: schematic::Schematic = INPUT.into();
    let partnumbers = schematic.calculate_partnumbers();
    let gear_ratios = schematic.calculate_gear_rations();

    println!("{}", schematic);
    println!("Part numbers: {}", partnumbers);
    println!("Gear ratios: {}", gear_ratios);
}

mod schematic {
    use colored::Colorize;
    use std::{collections::HashSet, hash::Hash};

    impl From<&str> for Schematic {
        fn from(input: &str) -> Self {
            let symbols: HashSet<char> = input.chars().filter(is_part_symbol).collect();
            let array = create_2d_array(input);
            let counted_numbers = HashSet::new();
            Schematic {
                array,
                symbols,
                counted_numbers,
            }
        }
    }

    fn is_part_symbol(c: &char) -> bool {
        !c.is_whitespace() && !c.is_numeric() && c != &'.'
    }

    fn create_2d_array(input: &str) -> Vec<Vec<char>> {
        let mut array = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c)
            }
            array.push(row);
        }
        array
    }

    pub struct Schematic {
        array: Vec<Vec<char>>,
        symbols: HashSet<char>,
        counted_numbers: HashSet<Point>,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Point {
        pub x: usize,
        pub y: usize,
    }

    impl Hash for Point {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.x.hash(state);
            self.y.hash(state);
        }
    }

    impl Schematic {
        fn get(&self, p: &Point) -> Option<&char> {
            self.array.get(p.y).and_then(|row| row.get(p.x))
        }

        // Returns a list of the beginnings of all adjacent numbers to the given point
        fn get_adjacent_numbers(&self, p: &Point) -> Vec<Point> {
            let mut adjacent_numbers: Vec<Point> = Vec::new();
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue; // Skip the center point
                    }
                    let new_x = p.x as i32 + i;
                    let new_y = p.y as i32 + j;

                    if new_x >= 0 && new_y >= 0 {
                        let p = Point {
                            x: new_x as usize,
                            y: new_y as usize,
                        };
                        if let Some(c) = self.get(&p) {
                            if c.is_numeric() {
                                let beginning_of_number = self.get_beginning_of_number(&p);
                                if adjacent_numbers.contains(&beginning_of_number) {
                                    continue;
                                }
                                adjacent_numbers.push(beginning_of_number);
                            }
                        }
                    }
                }
            }
            adjacent_numbers
        }

        fn get_beginning_of_number(&self, p: &Point) -> Point {
            let mut x = p.x;
            while let Some(c) = self.get(&Point { x: x, y: p.y }) {
                if !c.is_numeric() {
                    x += 1;
                    break;
                }
                if x == 0 {
                    break;
                }
                x -= 1;
            }
            Point { x: x, y: p.y }
        }

        fn is_symbol(&self, p: &Point) -> bool {
            match self.get(&p) {
                Some(c) => self.symbols.contains(c),
                None => false,
            }
        }

        fn get_number(&self, p: &Point) -> u32 {
            let mut number = String::new();
            let mut x = p.x;
            while let Some(c) = self.get(&Point { x: x, y: p.y }) {
                if !c.is_numeric() {
                    break;
                }
                number.push(*c);
                x += 1;
            }
            number.parse::<u32>().unwrap()
        }

        pub fn calculate_partnumbers(&mut self) -> u32 {
            let mut partnumbers = 0;
            for y in 0..self.array.len() {
                for x in 0..self.array[y].len() {
                    let p = Point { x: x, y: y };
                    if !self.is_symbol(&p) {
                        continue;
                    }

                    let adjacent_numbers = self.get_adjacent_numbers(&p);
                    for adjecent_number in adjacent_numbers.into_iter() {
                        // Check if the number has already been counted
                        if self.counted_numbers.contains(&adjecent_number) {
                            continue;
                        }

                        let number = self.get_number(&adjecent_number);
                        partnumbers += number;
                        self.counted_numbers.insert(adjecent_number);
                    }
                }
            }
            partnumbers
        }

        fn is_gear(&self, p: &Point) -> bool {
            let c = self.get(&p).unwrap();
            if *c != '*' {
                return false;
            }

            let adjacent_numbers = self.get_adjacent_numbers(&p);
            if adjacent_numbers.len() != 2 {
                return false;
            }

            if self.get_beginning_of_number(&adjacent_numbers[0])
                == self.get_beginning_of_number(&adjacent_numbers[1])
            {
                return false;
            }

            true
        }

        pub fn calculate_gear_rations(&self) -> u32 {
            let mut gear_ratio_sum = 0;

            self.array.iter().enumerate().for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, _)| {
                    let p = Point { x: x, y: y };
                    if !self.is_gear(&p) {
                        return;
                    }

                    let adjacent_numbers = self.get_adjacent_numbers(&p);
                    let mut gear_ratio = 1;
                    for adjecent_number in adjacent_numbers.iter() {
                        let number = self.get_number(&adjecent_number);
                        gear_ratio *= number;
                    }

                    gear_ratio_sum += gear_ratio;
                })
            });

            gear_ratio_sum
        }
    }

    impl std::fmt::Display for Schematic {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for (i, row) in self.array.iter().enumerate() {
                for (j, c) in row.iter().enumerate() {
                    if c.is_numeric() {
                        let beginning = self.get_beginning_of_number(&Point { x: j, y: i });
                        if self.counted_numbers.contains(&beginning) {
                            write!(f, "{}", c.to_string().green())?;
                        } else {
                            write!(f, "{}", c.to_string().blue())?;
                        }
                    } else if self.symbols.contains(c) {
                        if self.is_gear(&Point { x: j, y: i }) {
                            write!(f, "{}", c.to_string().yellow())?;
                        } else {
                            write!(f, "{}", c.to_string().red())?;
                        }
                    } else {
                        write!(f, "{}", c)?;
                    }
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_create_2d_array() {
            let input = "123\n456\n789";
            let expected = vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ];
            assert_eq!(create_2d_array(input), expected);

            let input = "1..#\n4...\n%*3.";
            let expected = vec![
                vec!['1', '.', '.', '#'],
                vec!['4', '.', '.', '.'],
                vec!['%', '*', '3', '.'],
            ];
            assert_eq!(create_2d_array(input), expected);
        }

        #[test]
        fn test_get_adjacent_numbers() {
            let input = "123\n456\n789";
            let schematic: Schematic = input.into();
            assert_eq!(
                schematic.get_adjacent_numbers(&Point { x: 0, y: 0 }),
                vec![
                    Point { x: 0, y: 1 },
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: 1 }
                ]
            );
            assert_eq!(
                schematic.get_adjacent_numbers(&Point { x: 1, y: 1 }),
                vec![
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 0, y: 2 },
                    Point { x: 1, y: 0 },
                    Point { x: 1, y: 2 },
                    Point { x: 2, y: 0 },
                    Point { x: 2, y: 1 },
                    Point { x: 2, y: 2 }
                ]
            );
            assert_eq!(
                schematic.get_adjacent_numbers(&Point { x: 2, y: 2 }),
                vec![
                    Point { x: 1, y: 1 },
                    Point { x: 1, y: 2 },
                    Point { x: 2, y: 1 }
                ]
            );
        }

        #[test]
        fn test_get_beginning_of_number() {
            let input = "123\n456\n789";
            let schematic: Schematic = input.into();
            assert_eq!(
                schematic.get_beginning_of_number(&Point { x: 0, y: 0 }),
                Point { x: 0, y: 0 }
            );
            assert_eq!(
                schematic.get_beginning_of_number(&Point { x: 1, y: 1 }),
                Point { x: 0, y: 1 }
            );

            let input = ".23\n4/6\n78%";
            let schematic: Schematic = input.into();
            assert_eq!(
                schematic.get_beginning_of_number(&Point { x: 2, y: 0 }),
                Point { x: 1, y: 0 }
            );
            assert_eq!(
                schematic.get_beginning_of_number(&Point { x: 1, y: 1 }),
                Point { x: 2, y: 1 }
            );
            assert_eq!(
                schematic.get_beginning_of_number(&Point { x: 1, y: 2 }),
                Point { x: 0, y: 2 }
            );
        }

        #[test]
        fn test_calculate_partnumbers() {
            let input = "10.\n/..\n...";
            let mut schematic: Schematic = input.into();
            assert_eq!(schematic.calculate_partnumbers(), 10);

            let input = "1..#\n4...\n%*3.";
            let mut schematic: Schematic = input.into();
            assert_eq!(schematic.calculate_partnumbers(), 12);
        }

        #[test]
        fn test_is_gear() {
            let input = ".1..\n.*..\n1...";
            let schematic: Schematic = input.into();
            assert_eq!(schematic.is_gear(&Point { x: 1, y: 1 }), true);

            let input = ".10.\n.*..\n1...";
            let schematic: Schematic = input.into();
            assert_eq!(schematic.is_gear(&Point { x: 1, y: 1 }), true);

            let input = "....\n.*..\n1...";
            let schematic: Schematic = input.into();
            assert_eq!(schematic.is_gear(&Point { x: 1, y: 1 }), false);
        }
    }
}
