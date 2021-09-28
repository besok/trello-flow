use std::cmp::max;

pub struct WordMatcher {}

impl WordMatcher {
    pub fn math_words(left: &str, right: &str, prob: f32) -> bool {
        if left == right { true } else {
            let mut idx = 1f32;
            for (l, r) in left.chars().zip(right.chars()) {
                if l != r { break; } else { idx += 1.0; }
            }
            idx / max(left.len(), right.len()) as f32 >= prob
        }
    }
}


mod tests {
    use crate::matcher::WordMatcher;

    #[test]
    fn test() {
        assert_eq!(WordMatcher::math_words("fuzzy word", "word", 0.8), false);
        assert_eq!(WordMatcher::math_words("fuzzy ", "fuzz", 0.8), true);
    }
}