pub fn _valid_anagram(s: &mut String, t: &mut String) -> bool {
    if unsafe { s.as_bytes_mut().sort() } == unsafe { t.as_bytes_mut().sort() } {
        return true;
    }
    false
}
