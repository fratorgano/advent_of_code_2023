# 🎄 Advent of Code 2023 🎄

I ended up not finishing last year's challenge and I probably won't finish this one either but here I am again!
The setup is basically the exact same as last year since it worked well enough, you can go read more [here](https://github.com/fratorgano/advent_of_code_2022#-new-this-year-)

I'm gonna stick to Rust as far as I can, I haven't used since last year so it might take a bit to get back up to speed

Bonus: since I'm following a game dev course I might do some days on C# and do a visualization using Unity! 🎮

🏆 Goal: do as much as I can! 🏆

[13/12] Update 1: I removed input file as requested by AoC creator, they should go into a folder "inputs" at the same level of the "crates" folder
The name format for each input file must be "dayXX.txt"

[18/12] Update 2: I started to build my own helper library so I can store some commonly used functions and structures, to use it add ```helper = {path="../../helper"}``` under [dependencies] in the Cargo.toml file of the day crate

## ❄️ How to use ❄️
`cargo run -p day**` - Runs a specific day

`cargo run -p day** --release` - Runs a specific day with compiler optimizations

`cargo test -p day**` - Tests a specific day

`cargo test` - Tests all

## 🥛 Results 🍪
| Day | Part 1 Time | Part 1 Rank | Part 1 Runtime[^1][^2] | Part 2 Time | Part 2 Rank | Part 2 Runtime[^1][^2] |
|:-:|-:|-:|-:|-:|-:|-:|
|  1 | 02:03:05 | 19532 |  82.2µs | 02:19:45 | 12618 | 862.5µs |
|  2 | 05:14:58 | 33890 |    -    | 05:26:05 | 31961 |    -    |
|  3 | 06:12:48 | 29487 |    -    | 06:52:24 | 25194 |    -    |
|  4 | 02:26:02 | 22772 |    -    | 02:53:55 | 18595 |    -    |
|  5 | 02:51:43 | 16743 |    -    | 04:30:26 |  9408 |    -    |
|  6 | 01:47:05 | 16182 |    -    | 01:52:56 | 15511 |    -    |
|  7 | 03:44:03 | 18166 |   2.6ms | 04:22:02 | 15396 |   5.1ms |
|  8 | 02:28:38 | 17569 | 800.5µs | 03:09:42 | 12020 |   3.6ms |
|  9 | 07:57:46 | 29871 | 463.3µs | 08:06:18 | 28979 | 492.8µs |
| 10 |     >24h | 44590 |   1.7ms |     >24h | 30422 |  89.5ms |
| 11 | 06:09:43 | 20218 | 273.0µs | 06:58:48 | 19273 | 216.5µs |
| 12 | 02:56:02 |  8672 |    -    | 09:01:01 |  7408 |    -    |  
| 13 | 03:39:30 | 10380 |   1.1ms | 04:20:17 |  8579 |  82.3ms |
| 14 | 03:15:55 | 12044 |    -    | 06:59:51 | 11467 |    -    |
| 15 | 02:05:22 | 10683 | 294.9µs | 02:42:08 |  9047 | 570.4µs |
| 16 | 12:38:58 | 19322 |    -    | 12:46:45 | 18197 |    -    |
| 17 |     >24h | 16934 |    -    |     >24h | 15994 |    -    |
| 18 | 10:00:43 | 13992 |    -    | 10:24:12 |  9151 |    -    |
| 19 | 03:44:41 |  8609 |    -    | 11:37:39 |  9386 |    -    |
| 20 | 04:19:51 |  5836 |   5.2ms | 05:03:43 |  3543 |  87.7ms |
| 21 | 07:40:40 | 12192 |    -    | 08:17:07 |  3636 |    -    |
<!--|  1 | 00:13:19 |  5740 |  19.5µs | 00:21:33 |  5187 |  20.7µs | -->

## 🎅 Have a Wonderful Holiday Season, Everyone! 🎅 

![koch flakes](https://raw.githubusercontent.com/fratorgano/advent_of_code_2020/main/snow.gif)


[^1]: `cargo run -p day** --release`, does not include the reading of the input file but includes parsing.
[^2]: Some values are missing since I solved that days without using the usual device I use
