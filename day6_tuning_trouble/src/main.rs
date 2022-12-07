use std::collections::HashSet;

fn main() {
    if let [_,_,_,(a,_)] = include_bytes!("data.txt")
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(4)
        .find(|f| f.into_iter().map(|(_, e)| e).collect::<HashSet<_>>().len() == 4 )
        .unwrap() {
            println!("part_one: {}", a+1);
        }

    if let [_, .. ,(a,_)] = include_bytes!("data.txt")
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(14)
        .find(|f| f.into_iter().map(|(_, e)| e).collect::<HashSet<_>>().len() == 14 )
        .unwrap() {
            println!("part_two: {}", a+1);
        }
}
