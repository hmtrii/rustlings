use std::collections::HashMap;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Progress {
    None,
    Some,
    Complete,
}

fn main() {
    use Progress::*;

    let mut map = HashMap::new();
    map.insert(String::from("variables1"), Complete);
    map.insert(String::from("functions1"), Complete);
    map.insert(String::from("hashmap1"), Complete);
    map.insert(String::from("arc1"), Some);
    map.insert(String::from("as_ref_mut"), None);
    map.insert(String::from("from_str"), None);
    let sum = map.iter().fold(0, |mut sum, (_, value)| {
        if *value == Complete {
            sum += 1;
        }
        sum
    });
    println!("{:?}", sum)
}