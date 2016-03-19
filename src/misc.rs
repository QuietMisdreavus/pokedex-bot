pub fn starts_with(s1: &str, s2: &str) -> bool {
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            return false;
        }
    }
    true
}
