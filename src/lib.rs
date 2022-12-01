pub mod day1 {
    pub fn part1(lines: &String) -> i32 {
        use std::cmp;

        let mut max = 0;
        let mut current = 0;

        for line in lines.lines() {
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

    pub fn part2(lines: &String) -> i32 {
        let mut top1 = 0;
        let mut top2 = 0;
        let mut top3 = 0;

        let mut current = 0;

        for line in lines.lines() {
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

        assert_eq!(crate::day1::part1(&String::from(input)), 24000)
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

        assert_eq!(crate::day1::part2(&String::from(input)), 45000)
    }
}
