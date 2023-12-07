use advent_of_code_2023::to_u32;

struct GameDraw {
    red: u32,
    blue: u32,
    green: u32,
}

struct Game {
    id: u32,
    draws: Vec<GameDraw>,
}

fn parse_game_draw(draw_str: &str) -> GameDraw {
    let (mut red, mut green, mut blue) = (0u32, 0u32, 0u32);
    let draw_str_elems = draw_str.split(",").map(|x| x.trim()).collect::<Vec<&str>>();

    for elem in draw_str_elems {
        if elem.ends_with("red") {
            red = to_u32(&elem.replace("red", "").trim())
        } else if elem.ends_with("green") {
            green = to_u32(&elem.replace("green", "").trim())
        } else if elem.ends_with("blue") {
            blue = to_u32(&elem.replace("blue", "").trim())
        } else {
            panic!(
                "Unrecognised element {} when parsing game draw {}",
                elem, draw_str
            );
        }
    }

    GameDraw { red, green, blue }
}

fn game_draw_is_valid(draw: &GameDraw) -> bool {
    draw.red <= 12 && draw.green <= 13 && draw.blue <= 14
}

fn build_game(game_str: &str) -> Game {
    let (id_str, result_str) = game_str.split_once(":").unwrap();
    let id = to_u32(&id_str.replace("Game ", ""));

    let draws = result_str
        .split(";")
        .map(|x| parse_game_draw(x))
        .collect::<Vec<GameDraw>>();

    Game { id, draws }
}

/// returns the Some(game_id) if the game is valid. None if invalid
fn game_is_valid(game_str: &str) -> Option<u32> {
    let game = build_game(game_str);

    let valid_games = game
        .draws
        .iter()
        .fold(0, |vg, draw| vg + game_draw_is_valid(draw) as usize);

    if game.draws.len() == valid_games {
        Some(game.id)
    } else {
        None
    }
}

fn calc_game_power(game_str: &str) -> u32 {
    let game = build_game(game_str);
    let max_red = game.draws.iter().max_by_key(|d| d.red).unwrap().red;
    let max_green = game.draws.iter().max_by_key(|d| d.green).unwrap().green;
    let max_blue = game.draws.iter().max_by_key(|d| d.blue).unwrap().blue;
    max_red * max_green * max_blue
}

pub fn part_a(input: &str) -> u32 {
    let games = input.split("\n").collect::<Vec<&str>>();
    games.iter().map(|x| game_is_valid(x)).flatten().sum()
}

pub fn part_b(input: &str) -> u32 {
    let games = input.split("\n").collect::<Vec<&str>>();
    games.iter().map(|x| calc_game_power(x)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(2);
        assert_eq!(part_a(&input), 8);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(2);
        assert_eq!(part_b(&input), 2286);
    }
}
