mod test;

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    min_occurrences: usize,
    max_occurrences: usize,
    value: String,
    password: String,
}

impl Rule {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split_ascii_whitespace().collect();
        let occurrences = parts
            .first()
            .unwrap_or(&"1-1")
            .split("-")
            .map(|x| x.parse::<usize>().unwrap_or(1))
            .collect::<Vec<usize>>();

        Rule {
            min_occurrences: *occurrences.first().unwrap(),
            max_occurrences: *occurrences.last().unwrap(),
            value: parts.iter().nth(1).unwrap_or(&"a").replace(":", ""),
            password: String::from(*parts.iter().last().unwrap_or(&"a")),
        }
    }

    fn is_valid_sled_password(&self) -> bool {
        let target = &self.value.chars().nth(0).unwrap();
        let len = self
            .password
            .chars()
            .filter(|x| x == target)
            .collect::<Vec<_>>()
            .len();

        len >= self.min_occurrences && len <= self.max_occurrences
    }

    fn is_valid_toboggan_password(&self) -> bool {
        let target = self.value.chars().nth(0).unwrap();
        self.password
            .char_indices()
            .filter(|x| {
                ((x.0 + 1 == self.min_occurrences) || (x.0 + 1 == self.max_occurrences))
                    && x.1 == target
            })
            .collect::<Vec<(usize, char)>>()
            .len()
            == 1
    }
}

pub fn solve(input: &str) {
    let answer_part_one = solve_part_one(&input);
    let answer_part_two = solve_part_two(&input);
    println!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

pub fn solve_both(input: &str) -> String {
    let answer_part_one = solve_part_one(input);
    let answer_part_two = solve_part_two(input);
    format!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

pub fn solve_part_one(input: &str) -> String {
    let rules = input
        .split("\n")
        .map(|x| Rule::new(x))
        .collect::<Vec<Rule>>();

    format!(
        "{}",
        rules
            .iter()
            .filter(|x| x.is_valid_sled_password())
            .collect::<Vec<&Rule>>()
            .len()
    )
}

pub fn solve_part_two(input: &str) -> String {
    let rules = input
        .split("\n")
        .map(|x| Rule::new(x))
        .collect::<Vec<Rule>>();

    format!(
        "{}",
        rules
            .iter()
            .filter(|x| x.is_valid_toboggan_password())
            .collect::<Vec<&Rule>>()
            .len()
    )
}
