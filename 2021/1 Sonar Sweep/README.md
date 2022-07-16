# Sonar Sweep

Checkout the puzzle [here](https://adventofcode.com/2021/day/1).

I was curious how long it took to run my code so I did a quick `cargo build --release` and benchmarked it with [hyperfine](https://github.com/sharkdp/hyperfine):

```r
$ hyperfine.exe .\sonar_sweep.exe --warmup 512 --runs 1024
    Time (mean ± σ):       9.0 ms ±   4.5 ms    [User: 0.1 ms, System: 0.1 ms]
    Range (min … max):     6.0 ms …  66.3 ms    1024 runs
```

Running over 2000 data points(Including the time to convert the large string the list of data points is stored as into a vec of seperate strings and then a vec of u32) in 9ms sounds pretty good to me!
