use scanf::sscanf;

const INPUT: &str = include_str!("input.txt");

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let mut games: Vec<Game> = Vec::new();
    INPUT.lines().for_each(|line| {
        let game = parse_line(line);
        match game {
            Some(game) => games.push(game),
            None => (),
        }
    });

    let id_sum = games.iter().fold(0, |acc, game| acc + game.id);

    println!("Sum of ids: {}", id_sum);
}

fn parse_line(line: &str) -> Option<Game> {
    let mut id: u32 = 0;
    let mut rest = String::new();
    sscanf!(line, "Game {}:{}", id, rest).unwrap();
    // Workaround for sscanf omitting a space for some reason
    rest = " ".to_owned() + &rest;

    let variants = get_game_variants(&rest);

    for variant in variants {
        match parse_variant(variant) {
            Some(_) => (),
            None => return None,
        }
    }

    Some(Game { id })
}

fn get_game_variants<'a>(game: &'a String) -> Vec<&'a str> {
    let mut variants: Vec<&str> = Vec::new();

    game.split(";").for_each(|variant| {
        variants.push(variant);
    });

    variants
}

fn parse_variant(variant: &str) -> Option<ColorCount> {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    let color_variants = variant.split(",");

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
            "red" => {
                if value > MAX_RED {
                    return None;
                }
                red += value
            }
            "green" => {
                if value > MAX_GREEN {
                    return None;
                }
                green += value
            }
            "blue" => {
                if value > MAX_BLUE {
                    return None;
                }
                blue += value
            }
            _ => panic!("Unknown color: {}", color),
        }
    }

    Some(ColorCount { red, green, blue })
}

struct Game {
    id: u32,
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
        let cc = parse_variant(variant).unwrap();

        assert_eq!(cc.red, 2);
        assert_eq!(cc.green, 0);
        assert_eq!(cc.blue, 0);

        let variant = " 2 red, 3 green";
        let cc = parse_variant(variant).unwrap();

        assert_eq!(cc.red, 2);
        assert_eq!(cc.green, 3);
        assert_eq!(cc.blue, 0);

        let variant = " 2 red, 3 green, 4 blue";
        let cc = parse_variant(variant).unwrap();

        assert_eq!(cc.red, 2);
        assert_eq!(cc.green, 3);
        assert_eq!(cc.blue, 4);

        let variant = " 14 red, 3 green, 4 blue";
        let cc: Option<ColorCount> = parse_variant(variant);
        match cc {
            Some(_) => panic!("Should not be able to parse this"),
            None => (),
        }
    }

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 2 green; 4 red";
        let game = parse_line(line).unwrap();

        assert_eq!(game.id, 1);

        let line = "Game 4: 2 red, 3 green, 4 blue; 5 red, 6 green, 7 blue";
        let game = parse_line(line).unwrap();

        assert_eq!(game.id, 4);

        let line = "Game 1: 2 red, 20 green, 4 blue";
        let game = parse_line(line);
        match game {
            Some(_) => panic!("Should not be able to parse this"),
            None => (),
        }
    }
}
