fn main() {
    use std::time::Instant;
    let mut now = Instant::now();

    println!(
        "part_one: {}",
        include_bytes!("data.txt")
            .split(|b| *b == b'\n')
            .map(|l| l.split_at(l.len() / 2))
            .map(|(a, b)| {
                b.iter()
                    .filter(|b| a.contains(b))
                    .map(|b| {
                        if *b >= b'a' {
                            return (b - b'a') as u16 + 1;
                        } else {
                            return (b - b'A') as u16 + 27;
                        }
                    })
                    .next()
                    .unwrap()
            })
            .sum::<u16>()
    );
    println!("part_one: {:?}", now.elapsed());

    now = Instant::now();
    println!(
        "part_two: {}",
        include_bytes!("data.txt")
            .split(|b| *b == b'\n')
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|a| {
                a[0].iter()
                    .filter(|x| a[1].contains(x) && a[2].contains(x))
                    .map(|b| {
                        if *b >= b'a' {
                            return (b - b'a') as u16 + 1;
                        } else {
                            return (b - b'A') as u16 + 27;
                        }
                    })
                    .next()
                    .unwrap()
            })
            .sum::<u16>()
    );
    println!("part_two: {:?}", now.elapsed());

    now = Instant::now();
    println!(
        "part_two: Optimal?: {}",
        include_bytes!("data.txt")
            .split(|b| *b == b'\n')
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|set| set[0]
                .iter()
                .find(|b| set[1].contains(b) && set[2].contains(b))
                .unwrap())
            .map(|b| if *b >= b'a' {
                (b - b'a') as i16 + 1
            } else {
                (b - b'A') as i16 + 27
            })
            .sum::<i16>(),
    );
    println!("part_two: Optimal?: {:?}", now.elapsed());
}
