pub fn part1(input:&[String]) -> usize {
  let races = parse(input);
  calculate_ways(races)
}
pub fn part2(input:&[String]) -> usize {
  let race = parse2(input);
  count_ways_to_win(race.0,race.1)
}

fn calculate_ways(races:Vec<(usize,usize)>) -> usize {
  // assuming there's always at least one way
  let mut ways_count = 1;
  for (time,record) in races {
    ways_count *= count_ways_to_win(time, record)
  }
  ways_count
}

fn count_ways_to_win(time:usize,record:usize) -> usize {
  let mut ways_count = 0;
  for i in 0..time {
    let new_distance = (time-i)*i;
    if new_distance>record { 
      ways_count+=1;
    }
  }
  ways_count
}

fn parse(strings:&[String]) -> Vec<(usize,usize)> {
  let times:Vec<usize> = strings[0].split(":").last().unwrap().split(" ").filter(|s|s.len()>0).map(|s|s.parse::<usize>().unwrap()).collect();
  let distances:Vec<usize> = strings[1].split(":").last().unwrap().split(" ").filter(|s|s.len()>0).map(|s|s.parse::<usize>().unwrap()).collect();
  let mut pair_vec = vec![];
  for i in 0..times.len() {
      pair_vec.push((times[i],distances[i]));
  }
  pair_vec
}

fn parse2(strings:&[String]) -> (usize,usize) {
  let times:String = strings[0].split(":").last().unwrap().split(" ").filter(|s|s.len()>0).fold(String::new(), |acc,s|acc+s);
  let distances:String = strings[1].split(":").last().unwrap().split(" ").filter(|s|s.len()>0).fold(String::new(), |acc,s|acc+s);
  (times.parse::<usize>().unwrap(),distances.parse::<usize>().unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "Time:      7  15   30
    Distance:  9  40  200".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(288, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(71503, part2(&get_input()));
  }
}