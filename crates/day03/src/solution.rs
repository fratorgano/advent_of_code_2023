use std::collections::{HashSet, HashMap};

// today's solution was not great, I might revise it later

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  // find_symbols(&parsed);
  find_part_numbers(&parsed)
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  find_gear_ratios(&parsed)
}

fn parse(strings:&[String]) -> Vec<Vec<char>> {
  let mut matrix = Vec::new();
  for string in strings {
    let mut row = Vec::new();
    for char in string.chars() {
      row.push(char);
    }
    matrix.push(row);
  }
  matrix
}

fn find_part_numbers(matrix: &Vec<Vec<char>>) -> usize {
  let mut total_part_numbers = 0;
  let mut number_string = String::new();
  let mut valid = false;
  for (i,row) in matrix.iter().enumerate() {
    for (j,c) in row.iter().enumerate() {
      if c.is_numeric() {
        number_string.push(*c);
        valid = if valid {true} else {check_validity(matrix, i, j)};
      } else {
        if valid {
          valid = false;
          total_part_numbers += number_string.parse::<usize>().unwrap();
        }
        number_string = String::new();
      }
    }
    if valid {
      total_part_numbers += number_string.parse::<usize>().unwrap();
    }
    number_string = String::new();
    valid = false;
  }
  total_part_numbers
}

fn find_gear_ratios(matrix: &Vec<Vec<char>>) -> usize {
  let mut number_string = String::new();
  let mut valid = false;
  let mut symbols_map:HashMap<(usize, usize), Vec<_>> = HashMap::new();
  let mut symbols = HashSet::new();
  for (i,row) in matrix.iter().enumerate() {
    for (j,c) in row.iter().enumerate() {
      if c.is_numeric() {
        number_string.push(*c);
        let symbol_option = check_validity_2(matrix, i, j);
        if symbol_option.is_some() {
          symbols.insert(symbol_option.unwrap());
          valid = true;
        }
      } else {
        if valid {
          valid = false;
          for symbol in symbols {
            // symbols_map.insert(symbol, number_string.parse::<usize>());
            if symbols_map.contains_key(&symbol) {
              let vector = symbols_map.get(&symbol).unwrap();
              let mut new_vec = vector.clone();
              new_vec.push(number_string.parse::<usize>().unwrap());
              symbols_map.insert(symbol, new_vec);
            } else {
              let mut new_vec = Vec::new();
              new_vec.push(number_string.parse::<usize>().unwrap());
              symbols_map.insert(symbol, new_vec);
            }
          }
          symbols = HashSet::new();
        }
        number_string = String::new();
      }
    }
    if valid {
      for symbol in symbols {
        // symbols_map.insert(symbol, number_string.parse::<usize>());
        if symbols_map.contains_key(&symbol) {
          let vector = symbols_map.get(&symbol).unwrap();
          let mut new_vec = vector.clone();
          new_vec.push(number_string.parse::<usize>().unwrap());
          symbols_map.insert(symbol, new_vec);
        } else {
          let mut new_vec = Vec::new();
          new_vec.push(number_string.parse::<usize>().unwrap());
          symbols_map.insert(symbol, new_vec);
        }
      }
      
    }
    symbols = HashSet::new();
    number_string = String::new();
    valid = false;
  }
  let mut gear_ratio = 0;
  for ((i,j),value) in symbols_map {
    if matrix[i][j]=='*' && value.len()==2 {
      gear_ratio += value[0]*value[1];
    }
  }
  gear_ratio
}

/* fn find_symbols(matrix: &Vec<Vec<char>>) {
  let mut symbols = HashSet::new();
  for row in matrix {
    for c in row {
      if !c.is_numeric() && *c!='.' {
        symbols.insert(c);
      }
    }
  }
  println!("{:?}",symbols);
} */

fn check_validity(matrix: &Vec<Vec<char>>,row:usize,column:usize) -> bool {
  let valid_symbols = ['@', '+', '*', '-', '#', '=', '&', '%', '$', '/'];
  // check sides
  if column>0 {
    // check left 
    if valid_symbols.contains(&matrix[row][column-1]) { return true; }
  }
  if column<matrix[row].len()-1 {
    // check right
    if valid_symbols.contains(&matrix[row][column+1]) { return true; }
  }
  //check up 
  if row>0 {
    // check directly up
    if valid_symbols.contains(&matrix[row-1][column]) { return true; }
    if column>0 {
      // check up left
      if valid_symbols.contains(&matrix[row-1][column-1]) { return true; }
    }
    if column<matrix[row].len()-1 {
      // check up right
      if valid_symbols.contains(&matrix[row-1][column+1]) { return true; }
    }
  }
  // check down
  if row<matrix.len()-1 {
    // check directly down
    if valid_symbols.contains(&matrix[row+1][column]) { return true; }
    if column>0 {
      // check down left
      if valid_symbols.contains(&matrix[row+1][column-1]) { return true; }
    }
    if column<matrix[row].len()-1 {
      // check up right
      if valid_symbols.contains(&matrix[row+1][column+1]) { return true; }
    }
  }
  false
}

fn check_validity_2(matrix: &Vec<Vec<char>>,row:usize,column:usize) -> Option<(usize,usize)> {
  let valid_symbols = ['@', '+', '*', '-', '#', '=', '&', '%', '$', '/'];
  // check sides
  if column>0 {
    // check left 
    if valid_symbols.contains(&matrix[row][column-1]) { return Some((row,column-1)); }
  }
  if column<matrix[row].len()-1 {
    // check right
    if valid_symbols.contains(&matrix[row][column+1]) { return Some((row,column+1)); }
  }
  //check up 
  if row>0 {
    // check directly up
    if valid_symbols.contains(&matrix[row-1][column]) {return Some((row-1,column)); }
    if column>0 {
      // check up left
      if valid_symbols.contains(&matrix[row-1][column-1]) { return Some((row-1,column-1)); }
    }
    if column<matrix[row].len()-1 {
      // check up right
      if valid_symbols.contains(&matrix[row-1][column+1]) { return Some((row-1,column+1)); }
    }
  }
  // check down
  if row<matrix.len()-1 {
    // check directly down
    if valid_symbols.contains(&matrix[row+1][column]) { return Some((row+1,column)); }
    if column>0 {
      // check down left
      if valid_symbols.contains(&matrix[row+1][column-1]) { return Some((row+1,column-1)); }
    }
    if column<matrix[row].len()-1 {
      // check up right
      if valid_symbols.contains(&matrix[row+1][column+1]) { return Some((row+1,column+1)); }
    }
  }
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(4361, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(467835, part2(&get_input()));
  }
}