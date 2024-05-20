impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        return s.as_bytes().windows(2).map(|w| w[0].abs_diff(w[1]) as i32).sum();
    }
}