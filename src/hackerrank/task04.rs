#[allow(dead_code)]
pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&score| {
            let rem = score % 5;
            if score >= 38 && rem >= 3 {
                score + (5 - rem)
            } else {
                score
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::grading_students;

    #[test]
    fn test_rounding_logic() {
        let input = [73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }

    #[test]
    fn test_no_rounding_threshold() {
        let input = [37, 0, 100];
        let expected = vec![37, 0, 100];
        assert_eq!(grading_students(&input), expected);
    }

    #[test]
    fn test_execution_validity() {
        let dummy = [84];
        assert_eq!(grading_students(&dummy), vec![85]);
    }
}