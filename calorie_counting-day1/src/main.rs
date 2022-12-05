fn main() {
    let (max, _) = include_str!("data.txt").lines().fold((0, 0), {
        |(max, curr_total), curr| match curr.parse::<i32>().unwrap_or_default() > 0 {
            true => (max, curr_total + curr.parse::<i32>().unwrap()),
            _ => (std::cmp::max(max, curr_total), 0),
        }
    });
    println!("part_one: {}", max);

    let mut a = include_str!("data.txt")
        .split("\n\n")
        .map(|a| a.lines().map(|line| line.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();
    a.sort_unstable();

    println!("{}", a.into_iter().rev().take(3).sum::<i32>());
}
