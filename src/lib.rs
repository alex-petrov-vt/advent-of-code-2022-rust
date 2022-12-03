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
}
