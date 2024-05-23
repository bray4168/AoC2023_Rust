#[cfg(test)]
mod tests {
    use crate::input;
    use crate::solve::Solve;
    use crate::day1::Solution1;
    use crate::day1::Solution2;

    #[test]
    fn solution_1_test() {
        let solution = Solution1{};
        assert_eq!(solution.solve(), 0);
    }

    #[test]
    fn solution_2_test() {
        let solution = Solution2{};
        assert_eq!(solution.solve(), 0);
    }
}