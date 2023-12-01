fn main() {
    let input = std::fs::read("src/day-1/input.txt").expect("failed to open file");

    let sum = part_one(&input);

    println!("Part one - {}", sum)
}

fn part_one(input: &mut Vec<u8>) -> u32 {
    let len = input.len();

    let mut buf = Vec::new();
    let mut sum = 0;

    for (k, i) in input.into_iter().enumerate() {
        let ch = char::from(i);

        match ch {
            '0'..='9' => {
                buf.push(ch.to_digit(10).unwrap());
            }
            _ => {}
        }

        if ch == '\n' || k == len {
            sum += format!("{}{}", buf[0], buf[buf.len() - 1])
                .parse::<u32>()
                .unwrap();
            buf.clear();
        }
    }

    sum
}

fn part_two(input: &Vec<u8>) -> u32 {
    let len = input.len();

    let mut buf = Vec::new();
    let mut sum = 0;
    let mut word_buffer = String::from("");

    for (k, i) in input.into_iter().enumerate() {
        let ch = char::from(i);

        match ch {
            '0'..='9' => {
                buf.push(ch.to_digit(10).unwrap());
            }
            _ => {
                let peek = input[i + 1];

                word_buffer.push(ch);
                // "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
                if word_buffer == "one" {}
            }
        }

        if ch == '\n' || k == len {
            sum += format!("{}{}", buf[0], buf[buf.len() - 1])
                .parse::<u32>()
                .unwrap();
            buf.clear();
        }
    }

    sum
}
