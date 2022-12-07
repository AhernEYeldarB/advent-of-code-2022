use benchmark::jobs;

fn main() {
    benchmarking::warm_up();

    let res = jobs()
        .into_iter()
        .map(|(job, name)| {
            (
                name,
                benchmarking::measure_function(|measurer| {
                    measurer.measure(|| {
                        job();
                    })
                })
                .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    res.into_iter().for_each(|(name, time)| {
        println!("> {name} Total runtime === {:?}!", time.elapsed());
    });
}
