fn main() {
    const X: usize = 9;

    if let [a, b] = include_str!("data.txt")
        .split("\n\n")
        .collect::<Vec<_>>()
        .as_slice()
    {
        let mut iters: Vec<_> = String::from(*a)
            .lines()
            .take(8)
            .map(|l| {
                l.chars()
                    .enumerate()
                    .filter(|(i, _)| vec![1, 5, 9, 13, 17, 21, 25, 29, 33].contains(i))
                    .map(|(_, a)| a)
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|n| n.into_iter())
            .collect();
        let mut inverted: Vec<Vec<char>> = (0..X)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|v| {
                v.into_iter()
                    .filter(|c| c != &' ')
                    .rev()
                    .collect::<Vec<char>>()
            })
            .collect();
        println!("{:#?}", inverted);

        b.lines()
            .map(|l| l.split(" ").skip(1).step_by(2).collect::<Vec<_>>())
            .into_iter()
            .for_each(|l| {
                let (n, f, t) = (
                    l[0].parse::<i32>().unwrap(),
                    l[1].parse::<usize>().unwrap(),
                    l[2].parse::<usize>().unwrap(),
                );
                (0..n).for_each(|_| {
                    let temp = inverted[f - 1].pop().unwrap();
                    inverted[t - 1].push(temp);
                });
            });

        println!("{:?}", inverted);
        println!(
            "{:?}",
            inverted
                .into_iter()
                .map(|a| *a.last().unwrap())
                .collect::<String>()
        );
    }
}
