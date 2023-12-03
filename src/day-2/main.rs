fn main() {
    let input = std::fs::read("src/day-2/input.txt").expect("could not read file");
    let one_sum = part_one(String::from_utf8(input).unwrap());

    println!("Sum of part one: {}", one_sum);
}

#[derive(Debug)]
enum State {
    None,
    Game,
    Round,
}

struct Round {
    green: u32,
    blue: u32,
    red: u32,
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn reset(&mut self) {
        self.id = 0;
        self.rounds = Vec::new();
    }
}

fn part_one(input: String) -> u32 {
    let mut buffer = String::from("");
    let mut sum = 0;
    let mut game = Game {
        id: 0,
        rounds: Vec::new(),
    };

    let mut state = State::None;
    let mut chars = input.chars().peekable();

    loop {
        let ch = chars.next().unwrap();

        match ch {
            '0'..='9' => {
                if matches!(state, State::None) {
                    buffer.clear();
                    state = State::Game;
                    game.rounds.push(Round {
                        red: 0,
                        blue: 0,
                        green: 0,
                    })
                }
            }
            ' ' => {
                if matches!(state, State::Round) {
                    let peek = chars.peek().unwrap();

                    match peek {
                        'r' => {
                            game.rounds.last_mut().unwrap().red += buffer.parse::<u32>().unwrap()
                        }
                        'g' => {
                            game.rounds.last_mut().unwrap().green += buffer.parse::<u32>().unwrap()
                        }
                        'b' => {
                            game.rounds.last_mut().unwrap().blue += buffer.parse::<u32>().unwrap()
                        }
                        _ => {}
                    }
                    buffer.clear();
                }
            }
            ':' => {
                state = State::Round;
                game.id = buffer.parse::<u32>().unwrap();
                buffer.clear();
            }
            '\n' => {
                let mut possible = true;

                for round in &game.rounds {
                    if round.red > 12 || round.green > 13 || round.blue > 14 {
                        possible = false;
                    }
                }

                if possible {
                    sum += game.id;
                }

                buffer.clear();
                game.reset();
                state = State::None;
            }
            ';' => game.rounds.push(Round {
                red: 0,
                blue: 0,
                green: 0,
            }),
            ',' => {
                buffer.clear();
            }
            _ => {}
        }

        if ch != ' ' && ch != ',' && ch != ';' && ch != ':' {
            buffer.push(ch)
        }

        if chars.peek().is_none() {
            break;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let input = String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
        );

        let one_sum = part_one(input);
        println!("Sum of part one: {}", one_sum)
    }
}
