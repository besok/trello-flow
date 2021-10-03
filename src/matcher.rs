use std::cmp::max;

pub struct WordMatcher {}

impl WordMatcher {
    fn iterate_with_zip(left: &str, right: &str, prob: f32) -> bool {
        let mut idx = left.chars().zip(right.chars()).take_while(|(l, r)| { l == r }).count();
        idx as f32 / right.len() as f32 >= prob
    }
    pub fn math_words(left: &str, right: &str, prob: f32) -> bool {
        if left.len() <= right.len() {
            WordMatcher::iterate_with_zip(left, right, prob)
        } else {
            WordMatcher::iterate_with_zip(right, left, prob)
        }
    }
}


mod tests {
    use crate::matcher::WordMatcher;

    #[test]
    fn test() {
        assert_eq!(WordMatcher::math_words("fuzzy word", "word", 0.8), false);
        assert_eq!(WordMatcher::math_words("fuzzy", "fuzz", 0.8), true);
        assert_eq!(WordMatcher::math_words("fuzyz", "fuzz", 0.8), false);
    }
}