pub mod day1 {
    pub fn part1(lines: std::str::Lines) -> i32 {
        use std::cmp;

        let mut max = 0;
        let mut current = 0;

        for line in lines {
            if line == "" {
                max = cmp::max(max, current);
                current = 0;
            } else {
                current += line.parse::<i32>().unwrap()
            }
        }
        max = cmp::max(max, current);

        max
    }

    pub fn part2(lines: std::str::Lines) -> i32 {
        let mut top1 = 0;
        let mut top2 = 0;
        let mut top3 = 0;

        let mut current = 0;

        for line in lines {
            if line == "" {
                (top1, top2, top3) = update_top_scores(current, top1, top2, top3);
                current = 0;
            } else {
                current += line.parse::<i32>().unwrap()
            }
        }
        (top1, top2, top3) = update_top_scores(current, top1, top2, top3);

        top1 + top2 + top3
    }

    fn update_top_scores(current: i32, top1: i32, top2: i32, top3: i32) -> (i32, i32, i32) {
        let mut new_top1 = top1;
        let mut new_top2 = top2;
        let mut new_top3 = top3;

        if current >= top1 {
            new_top3 = top2;
            new_top2 = top1;
            new_top1 = current;
        } else if current >= top2 {
            new_top3 = top2;
            new_top2 = current;
        } else if current >= top3 {
            new_top3 = current;
        }

        return (new_top1, new_top2, new_top3);
    }
}

pub mod day2 {
    #[derive(PartialEq, Eq)]
    enum Shape {
        Rock,
        Paper,
        Scissors,
    }

    impl Shape {
        fn shape_to_beat(&self) -> Shape {
            match self {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            }
        }

        fn shape_to_lose(&self) -> Shape {
            match self {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            }
        }
    }

    #[derive(PartialEq, Eq)]
    enum RoundOutcome {
        Loss,
        Draw,
        Win,
    }

    pub fn part1(lines: std::str::Lines) -> i32 {
        let mut score = 0;

        for line in lines {
            let mut split = line.split(' ');
            let opponent = split.next().unwrap();
            let me = split.next().unwrap();

            let outcome = play_round(char_to_shape(me), char_to_shape(opponent));

            score += shape_to_score(char_to_shape(me)) + outcome_to_score(outcome)
        }

        score
    }

    pub fn part2(lines: std::str::Lines) -> i32 {
        let mut score = 0;

        for line in lines {
            let mut split = line.split(' ');
            let opponent = split.next().unwrap();
            let target_outcome = split.next().unwrap();

            let shape_to_choose =
                determine_shape(char_to_outcome(target_outcome), char_to_shape(opponent));

            score +=
                shape_to_score(shape_to_choose) + outcome_to_score(char_to_outcome(target_outcome))
        }

        score
    }

    fn char_to_shape(c: &str) -> Shape {
        match c {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,

            // shouldn't happen
            _ => Shape::Rock,
        }
    }

    fn play_round(me: Shape, opponent: Shape) -> RoundOutcome {
        if me == opponent {
            return RoundOutcome::Draw;
        } else if me.shape_to_beat() == opponent {
            return RoundOutcome::Win;
        } else {
            return RoundOutcome::Loss;
        }
    }

    fn determine_shape(target_outcome: RoundOutcome, opponent: Shape) -> Shape {
        match target_outcome {
            RoundOutcome::Win => opponent.shape_to_lose(),
            RoundOutcome::Draw => opponent,
            RoundOutcome::Loss => opponent.shape_to_beat(),
        }
    }

    fn shape_to_score(s: Shape) -> i32 {
        match s {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn outcome_to_score(r: RoundOutcome) -> i32 {
        match r {
            RoundOutcome::Loss => 0,
            RoundOutcome::Draw => 3,
            RoundOutcome::Win => 6,
        }
    }

    fn char_to_outcome(c: &str) -> RoundOutcome {
        match c {
            "X" => RoundOutcome::Loss,
            "Y" => RoundOutcome::Draw,
            "Z" => RoundOutcome::Win,

            // shouldn't happen
            _ => RoundOutcome::Loss,
        }
    }
}

pub mod day3 {
    use std::collections::HashSet;

    pub fn part1(lines: std::str::Lines) -> i32 {
        let mut priority_score = 0;

        for line in lines {
            let left = &line[..(line.len() / 2)];
            let right = &line[(line.len() / 2)..];

            let common = find_common(left, right);
            priority_score += get_priority(common)
        }

        priority_score
    }

    pub fn part2(lines: std::str::Lines) -> i32 {
        let mut priority_score = 0;

        let mut current_lines: Vec<&str> = Vec::new();
        for (count, line) in lines.enumerate() {
            if count % 3 == 0 && count != 0 {
                let common = find_common_in_lines(&current_lines);
                priority_score += get_priority(common);

                current_lines.clear();
            }

            current_lines.push(line)
        }

        if current_lines.len() == 3 {
            let common = find_common_in_lines(&current_lines);
            priority_score += get_priority(common);
        }

        priority_score
    }

    fn find_common(left: &str, right: &str) -> char {
        let left_chars: HashSet<char> = HashSet::from_iter(left.chars());

        for c in right.chars() {
            if left_chars.contains(&c) {
                return c;
            }
        }

        ' '
    }

    fn find_common_in_lines(lines: &Vec<&str>) -> char {
        let first_chars: HashSet<char> = HashSet::from_iter(lines[0].chars());
        let second_chars: HashSet<char> = HashSet::from_iter(lines[1].chars());

        for c in lines[2].chars() {
            if first_chars.contains(&c) && second_chars.contains(&c) {
                return c;
            }
        }

        ' '
    }

    fn get_priority(c: char) -> i32 {
        if c >= 'a' && c <= 'z' {
            return c as i32 - 'a' as i32 + 1;
        } else if c >= 'A' && c <= 'Z' {
            return c as i32 - 'A' as i32 + 27;
        } else {
            return 0;
        }
    }
}

pub mod day4 {
    pub fn part1(lines: std::str::Lines) -> i32 {
        let mut pairs = 0;

        for line in lines {
            let (first, second) = parse_intervals(line);

            if fully_contains(first, second) {
                pairs += 1
            }
        }

        pairs
    }

    pub fn part2(lines: std::str::Lines) -> i32 {
        let mut overlaps = 0;

        for line in lines {
            let (first, second) = parse_intervals(line);

            if overlap(first, second) {
                overlaps += 1
            }
        }

        overlaps
    }

    fn parse_intervals(line: &str) -> ((i32, i32), (i32, i32)) {
        let mut intervals = line.split(',');
        let first_str = intervals.next().unwrap();
        let second_str = intervals.next().unwrap();

        let first: Vec<&str> = first_str.split('-').collect();
        let second: Vec<&str> = second_str.split('-').collect();

        (
            (
                first[0].parse::<i32>().unwrap(),
                first[1].parse::<i32>().unwrap(),
            ),
            (
                second[0].parse::<i32>().unwrap(),
                second[1].parse::<i32>().unwrap(),
            ),
        )
    }

    fn fully_contains(first: (i32, i32), second: (i32, i32)) -> bool {
        return (first.0 <= second.0 && first.1 >= second.1)
            || (second.0 <= first.0 && second.1 >= first.1);
    }

    fn overlap(first: (i32, i32), second: (i32, i32)) -> bool {
        return std::cmp::min(first.1, second.1) >= std::cmp::max(first.0, second.0);
    }
}

mod tests {
    #[test]
    fn day1_part1() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

        assert_eq!(crate::day1::part1(input.lines()), 24000)
    }

    #[test]
    fn day1_part2() {
        let input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

        assert_eq!(crate::day1::part2(input.lines()), 45000)
    }

    #[test]
    fn day2_part1() {
        let input = "\
A Y
B X
C Z
";

        assert_eq!(crate::day2::part1(input.lines()), 15)
    }

    #[test]
    fn day2_part2() {
        let input = "\
A Y
B X
C Z
";

        assert_eq!(crate::day2::part2(input.lines()), 12)
    }

    #[test]
    fn day3_part1() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

        assert_eq!(crate::day3::part1(input.lines()), 157)
    }

    #[test]
    fn day3_part2() {
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

        assert_eq!(crate::day3::part2(input.lines()), 70)
    }

    #[test]
    fn day4_part1() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

        assert_eq!(crate::day4::part1(input.lines()), 2)
    }

    #[test]
    fn day4_part2() {
        let input = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

        assert_eq!(crate::day4::part2(input.lines()), 4)
    }
}
