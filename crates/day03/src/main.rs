use std::env;
use std::fs;
use std::io::BufRead;
use std::time::Instant;

mod solution;

fn main() {
    // day03 
    let input = read_input("day03");

    let before1 = Instant::now();
    let res1 = solution::part1(&input);
    println!("Part 1 result: {} in {:?}",res1,before1.elapsed());

    let before2 = Instant::now();
    let res2 = solution::part2(&input);
    println!("Part 2 result: {} in {:?}",res2,before2.elapsed());
}

fn read_input(day:&str) -> Vec<String> {
    let mut dir = env::current_dir().unwrap();
    dir.push("inputs");
    dir.push(format!("{}.txt",day));
    let file = fs::File::open(dir).expect(&format!("File {day}.txt not found"));
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}
