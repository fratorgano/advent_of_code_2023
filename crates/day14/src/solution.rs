use std::collections::HashMap;

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  let mut scores = vec![];
  for _ in 0..parsed[0].len() {
    scores.push(0);
  }
  let new_dish = tilt(parsed,true);
  calc_score(&new_dish)
}

pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  let mut starting_dish = parsed;
  // print_matrix(&starting_dish);
  let mut map = HashMap::new();
  let mut found_repeat = false;
  let mut end_first_cycle = 0;
  let mut cycle = vec![];
  loop {
    let north_west_dish = tilt(starting_dish,true);
    //print_matrix(&north_west_dish);
    let north_north_dish = transpose(north_west_dish);
    //print_matrix(&north_north_dish);
    let west_west_dish = tilt(north_north_dish,true);
    //print_matrix(&west_west_dish);
    let north_west_dish = transpose(west_west_dish);
    //print_matrix(&north_west_dish);
    let south_east_dish = tilt(north_west_dish,false);
    //print_matrix(&south_east_dish);
    let south_south_dish = transpose(south_east_dish);
    //print_matrix(&south_south_dish);
    let east_east_dish = tilt(south_south_dish,false);
    //print_matrix(&east_east_dish);
    starting_dish = transpose(east_east_dish);
    let score = calc_score(&starting_dish);
    if found_repeat {
      // enter here only when the first cycle has been found and save elements in the cycle
      cycle.push(score);
    }
    if map.contains_key(&starting_dish) {
      // If I enter here again, I have found the cycle, I can leave
      if found_repeat {break}
      // If I enter here for the first time I have found the end of the first cycle
      found_repeat = true;
      end_first_cycle = map.len();
      map.clear();
    }
    map.insert(starting_dish.clone(),score);
  }
  cycle[(999999999-end_first_cycle-1)%cycle.len()]
}

pub fn transpose(inp_matrix:Vec<Vec<char>>) -> Vec<Vec<char>> {
  let matrix = (0..inp_matrix[0].len())
        .map(|i| inp_matrix.iter().map(|inner| inner[i].clone()).collect::<Vec<char>>())
        .collect();
  matrix
}

pub fn tilt(dish:Vec<Vec<char>>,north:bool) -> Vec<Vec<char>> {
  let mut new_dish = vec![];
  for column in dish {
    let new_column = tilt_col(&column,north);
    new_dish.push(new_column)
  }
  new_dish

}

pub fn tilt_col(column:&Vec<char>,north:bool) -> Vec<char> {
  let mut new_column = column.clone();
  let mut updated = true;
  while updated {
    updated = false;
    for i in 0..new_column.len()-1 {
      // println!("{}->{}",new_column[i+1],new_column[i]);
      let pair = if north {(new_column[i+1],new_column[i])} else {(new_column[i],new_column[i+1])};
      match pair {
        ('O','.') => {
          if north {
            new_column[i] = 'O';
            new_column[i+1] = '.';
          } else {
            new_column[i+1] = 'O';
            new_column[i] = '.';
          }
          updated = true;
        },
        _ => {}
      }
    }
  }
  // println!("{:?}",new_column);
  new_column
}

pub fn calc_score(dish:&Vec<Vec<char>>) -> usize {
  let mut scores = vec![];
  for _ in 0..dish[0].len() {
    scores.push(0);
  }
  for column in dish {
    for (i,elem) in column.iter().enumerate() {
      if *elem=='O' {
        scores[i]+=1;
      }
    }
  }
  scores.iter().enumerate().map(|(i,elem)|(scores.len()-i)*elem).sum::<usize>()
}

fn parse(strings:&[String]) -> Vec<Vec<char>> {
  let mut columns = vec![];
  for _ in 0..strings[0].len() {
    columns.push(vec![]);
  }
  for line in strings.iter() {
    for (i,elem) in line.chars().enumerate() {
      columns[i].push(elem);
    }
  }
  columns
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(136, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(64, part2(&get_input()));
  }
}