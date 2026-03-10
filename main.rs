fn words_to_marks(s: &str) -> u32 {
    s.chars()
        .map(|c| ((c as u8) - b'a' + 1) as u32)
        .sum()
}
