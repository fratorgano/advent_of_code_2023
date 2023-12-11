use std::vec;

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  let galaxies = find_expanded_galaxies(parsed,1);
  calc_pair_distance(galaxies)
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  let galaxies = find_expanded_galaxies(parsed,999_999);
  calc_pair_distance(galaxies)
}

fn calc_pair_distance(galaxies: Vec<(usize,usize)>) -> usize {
  let mut total_distance = 0;
  let mut galaxies_clone = galaxies.clone();
  for galaxy in galaxies.iter() {
    galaxies_clone.remove(0);
    for galaxy_2 in galaxies_clone.clone() {
      total_distance += usize::abs_diff(galaxy.0, galaxy_2.0) + usize::abs_diff(galaxy.1, galaxy_2.1)
    }
  }
  total_distance
}

fn find_expanded_galaxies(universe:Vec<Vec<bool>>,expansion_factor:usize) -> Vec<(usize,usize)> {
  let universe_size = (universe.len(),universe[0].len());
  // find rows to expand rows
  let mut rows_to_be_expanded = vec![];
  for (i,row) in (&universe).iter().enumerate() {
    let mut empty = true;
    for elem in row {
      if *elem {
        empty = false;
        break;
      }
    }
    if empty {
      rows_to_be_expanded.push(i)
    }
  }
  // find columns to expand
  let mut columns_to_be_expanded = vec![];
  for c in 0..universe_size.1 {
    let mut empty = true;
    for r in 0..universe_size.0 {
      if universe[r][c] {
        empty=false;
        break;
      }
    }
    if empty {
      columns_to_be_expanded.push(c);
    }
  }
  // calculate position of galaxies, considering expansion
  let mut expanded_galaxies = vec![];
  for i in 0..universe_size.0 {
    for j in 0..universe_size.1 {
      if universe[i][j] {
        let row_expansions = rows_to_be_expanded.iter().filter(|x|i>**x).count();
        let col_expansions = columns_to_be_expanded.iter().filter(|y|j>**y).count();
        expanded_galaxies.push((i+row_expansions*expansion_factor,j+col_expansions*expansion_factor))
      }
    }
  }
  expanded_galaxies
}

fn parse(strings:&[String]) -> Vec<Vec<bool>> {
  let mut space_image = vec![];
  for line in strings {
    let mut space_line = vec![];
    for c in line.chars() {
      space_line.push(if c=='.' {false} else {true});
    }
    space_image.push(space_line);
  }
  space_image
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(374, part1(&get_input()));
  }
}