use std::collections::HashMap;

mod arrays;
fn main() {
    let mut hm = HashMap::new();
    hm.insert(11, "eleven");
    hm.insert(22, "twelve");

    println!(
        "{:?}",
        &hm.get_key_value(&33).get_or_insert((&33, &"thirteen"))
    );

    let a = "b";
    println!("{:?}", a.as_bytes());

    println!(
        "{:#?}",
        arrays::group_anagram::group_anagram::_group_anagrams(vec![
            "ant".to_string(),
            "tan".to_string(),
            "do".to_string(),
            "od".to_string(),
            "ran".to_string()
        ])
    )
}
