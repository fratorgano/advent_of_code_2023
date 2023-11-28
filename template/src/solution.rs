pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  1
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  1
}

fn parse(strings:&[String]) -> Vec<usize> {
  vec![]
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(42, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(42, part2(&get_input()));
  }
}