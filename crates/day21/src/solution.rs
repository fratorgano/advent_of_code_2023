use std::collections::{HashSet, VecDeque};

// Looked up some hints from the advent of code subreddit and from an aoc telegram group

pub fn part1(input:&[String]) -> usize {
  let (start,matrix) = parse(input);
  count_reachable(start, &matrix, 64)
}
pub fn part2(input:&[String]) -> usize {
  let (start,matrix) = parse(input);
  count_reachable_infinite(start, &matrix)
}

fn count_reachable_infinite(start:(i32,i32),matrix:&Vec<Vec<char>>) -> usize {
  // grid is 131 characters wide
  // we want to walk up to 26501365 squares: 
  // 202300 times the length of the grid + 65
  // 65 = number of steps to reach the edge from the starting point

  let mut progression = Vec::new();
  for i in 0..3 {
    progression.push(count_reachable(start, matrix, 65+i*131));
  }
  while progression.len() < (26501365-65)/131 {
    progression.push(extrapolate_last_val(&progression));
  }
  extrapolate_last_val(&progression)
}

fn extrapolate_last_val(values: &Vec<usize>) -> usize {
  let differences = values[1..].iter().enumerate().map(|(i, x)| x - values[i]).collect::<Vec<_>>();
  if differences.iter().all(|x| x == &0) {
    *values.iter().last().unwrap()
  } else {
    values.iter().last().unwrap() + extrapolate_last_val(&differences)
  }
}

fn count_reachable(start:(i32,i32),matrix:&Vec<Vec<char>>,depth:usize) -> usize {
  let mut already_visited = HashSet::new();
  let mut neighbours = VecDeque::new();
  let mut last_step_neighbours = HashSet::new();
  neighbours.push_back((start,depth));
  let limits = (matrix[0].len() as i32,matrix[1].len() as i32);
  while let Some((position, steps)) = neighbours.pop_front() {
    if steps == 0 {
      last_step_neighbours.insert(position);
      continue;
    }
    if already_visited.contains(&(position,steps)) {
      continue;
    }
    already_visited.insert((position,steps));
    
    for neighbour in find_neighbours(position) {
      let x = neighbour.0.rem_euclid(limits.0);
      let y = neighbour.1.rem_euclid(limits.1);
      if matrix[y as usize][x as usize]=='.' {
        neighbours.push_back((neighbour,steps-1));
      }
    }
  }
  last_step_neighbours.len()
}

fn find_neighbours(element:(i32,i32)) -> Vec<(i32,i32)> {
  let mut neighbours = vec![];
  neighbours.push((element.0-1,element.1));
  neighbours.push((element.0+1,element.1));
  neighbours.push((element.0,element.1-1));
  neighbours.push((element.0,element.1+1));
  neighbours
}

fn parse(strings:&[String]) -> ((i32,i32),Vec<Vec<char>>) {
  let mut matrix = vec![];
  let mut start:(i32,i32) = (0,0);
  for (i,line) in strings.iter().enumerate() {
    let mut row = vec![];
    for (j,elem) in line.chars().enumerate() {
      if elem=='S' {
        start = (i as i32,j as i32);
        row.push('.');
        continue;
      }
      row.push(elem);
    }
    matrix.push(row);
  }
  (start,matrix)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    let (start,matrix) = parse(&get_input());
    let res = count_reachable(start, &matrix, 6);
    assert_eq!(16, res);
  }
  #[test]
  fn test_part1_10steps() {
    let (start,matrix) = parse(&get_input());
    let res = count_reachable(start, &matrix, 10);
    assert_eq!(50, res);
  }
  #[test]
  fn test_part1_50steps() {
    let (start,matrix) = parse(&get_input());
    let res = count_reachable(start, &matrix, 50);
    assert_eq!(1594, res);
  }
  #[test]
  fn test_part1_100steps() {
    let (start,matrix) = parse(&get_input());
    let res = count_reachable(start, &matrix, 100);
    assert_eq!(6536, res);
  }

  #[test]
  fn test_part1_500steps() {
    let (start,matrix) = parse(&get_input());
    let res = count_reachable(start, &matrix, 500);
    assert_eq!(167004, res);
  }

  /* #[test]
  fn test_part2() {
    assert_eq!(42, part2(&get_input()));
  } */
}