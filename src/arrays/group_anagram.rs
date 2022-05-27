pub mod group_anagram {
    pub fn _group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut group = std::collections::HashMap::new();
        for s in strs {
            let mut counter = [0_u8; 26];
            for &byte in s.as_bytes() {
                counter[(byte - b'a') as usize] += 1;
            }
            group.entry(counter).or_insert_with(Vec::new).push(s);
        }
        group.into_iter().map(|(_k, v)| v).collect()
    }
}
