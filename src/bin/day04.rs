use std::fs::read_to_string;

fn main() {
    let input_lines: Vec<String> = read_to_string("input/day04.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    println!("part1: {}", part1(&input_lines));
    println!("part2: {}", part2(&input_lines));
}

fn part1(input_lines: &[String]) -> usize {
    load_cards(input_lines)
        .into_iter()
        .map(|c| {
            if c.matches == 0 {
                0
            } else {
                1 << (c.matches - 1)
            }
        })
        .sum()
}

fn part2(input_lines: &[String]) -> usize {
    let mut cards = load_cards(input_lines);
    for id in 0..cards.len() {
        let Card { matches, count } = cards[id];
        for i in id + 1..id + 1 + matches {
            cards[i].count += count;
        }
    }
    cards.into_iter().map(|c| c.count).sum()
}

fn load_cards(input_lines: &[String]) -> Vec<Card> {
    let mut cards = Vec::new();
    for line in input_lines {
        let (_, numbers) = line.split_once(":").unwrap();
        let (winning_line, mine_line) = numbers.split_once("|").unwrap();
        let winning = parse_numline(winning_line);
        let mine = parse_numline(mine_line);
        let mut matches = 0;
        for my_num in mine {
            if winning.contains(&my_num) {
                matches += 1;
            }
        }
        cards.push(Card { count: 1, matches });
    }
    cards
}

fn parse_numline(line: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let mut start = 0;
    while start + 3 <= line.len() {
        let field = &line[start..start + 3];
        result.push(str::parse(field.trim()).unwrap());
        start += 3;
    }
    result
}

struct Card {
    count: usize,
    matches: usize,
}
