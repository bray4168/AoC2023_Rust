#[cfg(test)]
mod tests {
    use crate::solve::Solve;
    use crate::day2::Solution1;
    use crate::day2::Solution2;
    use crate::day2::solution1;
    use crate::day2::solution2;

    #[test]
    fn solution_1_test() {
        let solution = Solution1{};
        let answer = 2377;
        
        assert_eq!(solution.solve(), answer);
    }

    #[test]
    fn solution_1_test_basic() {
        let input: Vec<String> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        let answer = 8;
        
        assert_eq!(solution1::solution(input), answer);
    }

    #[test]
    fn solution_2_test() {
        let solution = Solution2{};
        let answer = 0;
        
        assert_eq!(solution.solve(), answer);
    }
}