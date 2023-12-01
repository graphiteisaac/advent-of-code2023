fn main() {
    let mut input = std::fs::read("src/day-1/input.txt").expect("failed to open file");

    let sum = part_one(&mut input);

    println!("Part one - {}", sum);

    let sum_two = part_two(&input);

    println!("Part two - {}", sum_two)
}

fn part_one(input: &mut Vec<u8>) -> u32 {
    let len = input.len();

    let mut buf = Vec::new();
    let mut sum = 0;

    for (k, i) in input.into_iter().enumerate() {
        let ch = char::from(*i);

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

    for (k, i) in input.into_iter().enumerate() {
        let ch = char::from(*i);

        match ch {
            '0'..='9' => {
                buf.push(ch.to_digit(10).unwrap());
            }
            'o' => {
                if k + 2 <= len && input[k..k + 3] == *"one".as_bytes() {
                    buf.push(1)
                }
            }
            't' => {
                if k + 2 <= len && input[k..k + 3] == *"two".as_bytes() {
                    buf.push(2)
                } else if k + 5 <= len && input[k..k + 5] == *"three".as_bytes() {
                    buf.push(3)
                }
            }
            'f' => {
                if k + 4 <= len && input[k..k + 4] == *"four".as_bytes() {
                    buf.push(4)
                } else if k + 3 <= len && input[k..k + 4] == *"five".as_bytes() {
                    buf.push(5)
                }
            }
            's' => {
                if k + 3 <= len && input[k..k + 3] == *"six".as_bytes() {
                    buf.push(6)
                } else if k + 5 <= len && input[k..k + 5] == *"seven".as_bytes() {
                    buf.push(7)
                }
            }
            'e' => {
                if k + 5 <= len && input[k..k + 5] == *"eight".as_bytes() {
                    buf.push(8)
                }
            }
            'n' => {
                if k + 4 <= len && input[k..k + 4] == *"nine".as_bytes() {
                    buf.push(9)
                }
            }
            _ => {}
        }

        if (ch == '\n' || k == len - 1) && buf.len() > 0 {
            sum += format!("{}{}", buf[0], buf.last().unwrap())
                .parse::<u32>()
                .unwrap();
            buf.clear();
        }
    }

    sum
}
