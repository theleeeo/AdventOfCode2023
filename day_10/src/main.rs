#![warn(clippy::all, clippy::pedantic)]

mod pipe_map;

use crate::pipe_map::parse_map;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let map = parse_map(INPUT);
    let dirs = map.get_first_directions();
    println!("{:?}", dirs);

    let mut pos_1 = map.start;
    let mut dir_1 = dirs[0];

    let mut pos_2 = map.start;
    let mut dir_2 = dirs[1];

    let mut steps = 0;
    loop {
        steps += 1;
        pos_1 = map.step(pos_1, &dir_1);
        dir_1 = map
            .get(pos_1.0, pos_1.1)
            .unwrap()
            .out_direction(&dir_1)
            .unwrap_or_else(|| panic!("{}: {:?}", steps, pos_1));

        pos_2 = map.step(pos_2, &dir_2);
        dir_2 = map
            .get(pos_2.0, pos_2.1)
            .unwrap()
            .out_direction(&dir_2)
            .unwrap_or_else(|| panic!("{}: {:?} {:?}", steps, pos_1, pos_2));
        if pos_1 == pos_2 {
            break;
        }
    }

    println!("{}: {:?} {:?}", steps, pos_1, pos_2);
}
