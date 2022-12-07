pub fn main() {
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
}
