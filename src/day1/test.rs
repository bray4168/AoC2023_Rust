#[cfg(test)]
mod tests {
    use crate::input;
    use crate::solve::Solve;
    use crate::day1::Solution1;
    use crate::day1::Solution2;
    use crate::day1::solution2::solution;

    #[test]
    fn solution_1_test() {
        let solution = Solution1{};
        assert_eq!(solution.solve(), 55477);
    }

    #[test]
    fn solution_2_test_input() {
        let mut input: Vec<String> = vec![];
        input::read_file(&"src/day1/test_input.txt", &mut input).unwrap();

        assert_eq!(solution(input), 281);
    }

    // Test the issue where the same string could combine two numbers i.e. oneight
    #[test]
    fn solution_2_test_combined_numbers() {
        let input: Vec<String> = vec!["oneeight".to_string(), "oneight".to_string(), "eighthree".to_string(), "sevenine".to_string()];
        
        assert_eq!(solution(input), 198);
    }

    #[test]
    fn solution_2_test() {
        let solution = Solution2{};
        assert_eq!(solution.solve(), 54431);
    }
}