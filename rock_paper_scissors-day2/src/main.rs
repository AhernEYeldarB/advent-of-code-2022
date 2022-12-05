fn main() {
    println!(
        "part_one: {}",
        include_str!("data.txt")
            .lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .into_iter()
            .fold(0, |accum, curr| -> i32 {
                match curr.as_slice() {
                    ["A", "X"] => accum + 1 + 3,
                    ["A", "Y"] => accum + 2 + 6,
                    ["A", "Z"] => accum + 3 + 0,
                    ["B", "X"] => accum + 1 + 0,
                    ["B", "Y"] => accum + 2 + 3,
                    ["B", "Z"] => accum + 3 + 6,
                    ["C", "X"] => accum + 1 + 6,
                    ["C", "Y"] => accum + 2 + 0,
                    ["C", "Z"] => accum + 3 + 3,
                    _ => unreachable!(),
                }
            })
    );

    println!(
        "part_two {}",
        include_str!("data.txt")
            .lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .into_iter()
            .fold(0, |accum, curr| -> i32 {
                match curr.as_slice() {
                    ["A", "X"] => accum + 3 + 0,
                    ["A", "Y"] => accum + 1 + 3,
                    ["A", "Z"] => accum + 2 + 6,
                    ["B", "X"] => accum + 1 + 0,
                    ["B", "Y"] => accum + 2 + 3,
                    ["B", "Z"] => accum + 3 + 6,
                    ["C", "X"] => accum + 2 + 0,
                    ["C", "Y"] => accum + 3 + 3,
                    ["C", "Z"] => accum + 1 + 6,
                    _ => unreachable!(),
                }
            })
    );
}
