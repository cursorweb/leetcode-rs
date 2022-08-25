// https://leetcode.com/problems/ransom-note/
pub fn ransom(ransom_note: String, magazine: String) -> bool {
    let mut x: Vec<char> = magazine.chars().collect();
    ransom_note.chars().all(|c| {
        if x.contains(&c) {
            x.remove(x.iter().position(|x| *x == c).unwrap());
            true
        } else {
            false
        }
    })
}

// that stranger sucks my code was better >:(
use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut x: HashMap<char, i32> = HashMap::new();
    for c in ransom_note.chars() {
        if x.contains_key(&c) {
            x.insert(c, x[&c] + 1);
        } else {
            x.insert(c, 1);
        }
    }

    for c in magazine.chars() {
        if x.contains_key(&c) {
            x.insert(c, x[&c] - 1);
        }
    }

    x.into_iter().all(|(_, n)| n <= 0)
}