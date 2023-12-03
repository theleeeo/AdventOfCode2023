use scanf::sscanf;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut games: Vec<Game> = Vec::new();
    INPUT.lines().for_each(|line| {
        let game = parse_line(line);
        games.push(game);
    });

    let mut power_sum: u32 = 0;
    for game in games {
        power_sum += game.max_red * game.max_green * game.max_blue;
    }

    println!("Sum of power: {}", power_sum);
}

fn parse_line(line: &str) -> Game {
    let mut id: u32 = 0;
    let mut rest = String::new();
    sscanf!(line, "Game {}:{}", id, rest).unwrap();
    // Workaround for sscanf omitting a space for some reason
    rest = " ".to_owned() + &rest;

    let variants = get_game_variants(&rest);

    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;

    for variant in variants {
        let cc = parse_variant(variant);

        if cc.red > max_red {
            max_red = cc.red;
        }
        if cc.green > max_green {
            max_green = cc.green;
        }
        if cc.blue > max_blue {
            max_blue = cc.blue;
        }
    }

    Game {
        max_red,
        max_green,
        max_blue,
    }
}

fn get_game_variants(game: &str) -> Vec<&str> {
    let mut variants: Vec<&str> = Vec::new();

    game.split(';').for_each(|variant| {
        variants.push(variant);
    });

    variants
}

fn parse_variant(variant: &str) -> ColorCount {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    let color_variants = variant.split(',');

    for cv in color_variants {
        let mut color = String::new();
        let mut value: u32 = 0;

        match sscanf!(cv, " {} {}", value, color) {
            Ok(_) => (),
            Err(err) => {
                panic!("Error parsing colorvariant: {}, error: {}", cv, err);
            }
        }

        match color.as_str() {
            "red" => red += value,
            "green" => green += value,
            "blue" => blue += value,
            _ => panic!("Unknown color: {}", color),
        }
    }

    ColorCount { red, green, blue }
}

#[derive(Debug)]
struct Game {
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

struct ColorCount {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_game_variants() {
        let game = String::from(" 1; 2; 3");
        let variants = get_game_variants(&game);

        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0], " 1");
        assert_eq!(variants[1], " 2");
        assert_eq!(variants[2], " 3");

        let game = String::from(" red 2; green 3");
        let variants = get_game_variants(&game);

        assert_eq!(variants.len(), 2);
        assert_eq!(variants[0], " red 2");
        assert_eq!(variants[1], " green 3");
    }

    #[test]
    fn test_parse_variant() {
        let variant = " 2 red";
        let cc = parse_variant(variant);

        assert_eq!(cc.red, 2);
        assert_eq!(cc.green, 0);
        assert_eq!(cc.blue, 0);

        let variant = " 2 red, 3 green";
        let cc = parse_variant(variant);

        assert_eq!(cc.red, 2);
        assert_eq!(cc.green, 3);
        assert_eq!(cc.blue, 0);

        let variant = " 2 red, 3 green, 4 blue";
        let cc = parse_variant(variant);

        assert_eq!(cc.red, 2);
        assert_eq!(cc.green, 3);
        assert_eq!(cc.blue, 4);

        let variant = " 14 red, 3 green, 4 blue";
        let cc = parse_variant(variant);

        assert_eq!(cc.red, 14);
        assert_eq!(cc.green, 3);
        assert_eq!(cc.blue, 4);
    }

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 2 red, 20 green, 4 blue";
        let game = parse_line(line);

        assert_eq!(game.max_red, 2);
        assert_eq!(game.max_green, 20);
        assert_eq!(game.max_blue, 4);

        let line = "Game 2: 2 red, 20 green, 4 blue; 3 red, 5 green, 6 blue";
        let game = parse_line(line);

        assert_eq!(game.max_red, 3);
        assert_eq!(game.max_green, 20);
        assert_eq!(game.max_blue, 6);
    }
}
