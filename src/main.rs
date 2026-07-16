use std::{collections::HashMap, fs};

fn main() {
    // 1. Our source text
    let path = "input.txt";
    let text = fs::read(path).expect("I am unable to find the File please check the path");
    let mut pair_counts: HashMap<(u8, u8), u32> = HashMap::new();

    for pairs in text.windows(2) {
        let key = (pairs[0], pairs[1]);
        let count = pair_counts.entry(key).or_insert(0);
        *count += 1;
    }

    let mut pairs_vec: Vec<(&(u8, u8), &u32)> = pair_counts.iter().collect();

    pairs_vec.sort_by(|a, b| b.1.cmp(a.1));

    // 4. Print the final results
    for (pairs, counts) in pairs_vec.iter() {
        let a = pairs.0 as char;
        let b = pairs.1 as char;
        println!("('{}','{}') , {}", a, b, counts);
    }
}
