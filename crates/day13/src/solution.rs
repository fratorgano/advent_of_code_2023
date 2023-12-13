use std::{collections::HashSet, vec};

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  let mut total = 0;
  for field in parsed {
    let verticals = field.find_vertical_reflection();
    if !verticals.is_empty() {
      total += verticals.iter().sum::<usize>();
    } else {
      let horizontals = field.find_horizontal_reflection();
      total += (horizontals.iter().sum::<usize>())*100
    }
  }
  total
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  let mut total = 0;
  for field in parsed {
    match field.clean() {
      (Some(x),None) => total+=x,
      (None,Some(x)) => total+=x*100,
      _ => panic!("Unexpected output of clean method")
    }
  }
  total
}
#[derive(Debug)]
pub struct Field {
  matrix:Vec<Vec<char>>
}
impl Field {
  // reflect on a vertical line
  fn find_vertical_reflection(&self) ->Vec<usize> {
    let matrix = self.matrix.clone();
    let mut reflections = HashSet::new();
    for (k,line) in matrix.iter().enumerate() {
      let mut possible_reflections = HashSet::new();
      for i in 1..line.len() {
        let (left,right) = line.split_at(i);
        let mut left = left.to_owned();
        let mut right = right.to_owned();
        // cap length to shortest and reverse left string
        if left.len()>right.len() {
          left.reverse();
          left = (&left[..right.len()]).to_vec()
        } else {
          left.reverse();
          right = (&right[..left.len()]).to_vec()
        }
        // println!("{}-> {:?}   {:?} -> {}",i,left,right,left.eq(&right));
        if left.eq(&right) {
          possible_reflections.insert(i);
        }
      }
      if possible_reflections.is_empty() {
        // println!("No possible reflections -> None");
        return vec![];
      } else {
        if k==0 {
          reflections = possible_reflections.to_owned();
        } else {
          reflections = reflections.intersection(&possible_reflections).map(|x|*x).collect();
          if reflections.is_empty() {
            // println!("No common reflections -> None");
            return vec![];
          }
        }
      }
    }
    return reflections.into_iter().collect();
  }

  fn find_horizontal_reflection(&self) ->Vec<usize> {
    let transposed = self.transpose();
    transposed.find_vertical_reflection()
  }

  fn transpose(&self) -> Field{
    assert!(!self.matrix.is_empty());
    let matrix = (0..self.matrix[0].len())
        .map(|i| self.matrix.iter().map(|inner| inner[i].clone()).collect::<Vec<char>>())
        .collect();
    Field { matrix }
  }
  fn clean(&self) -> (Option<usize>,Option<usize>) {
    let older_vertical = self.find_vertical_reflection().get(0).cloned();
    let older_horizontal = self.find_horizontal_reflection().get(0).cloned();
    for i in 0..self.matrix.len() {
      for j in 0..self.matrix[0].len() {
        let new_field = self.copy_with_replaced(i, j);
        let verticals = new_field.find_vertical_reflection();
        if verticals.len()>0 && older_vertical.is_none() {
          return (Some(verticals[0]),None);
        }
        if let Some(vertical) = older_vertical {
          for elem in verticals {
            if elem!=vertical {return (Some(elem),None)}
          }
        }
        let horizontals = new_field.find_horizontal_reflection();
        if horizontals.len()>0 && older_horizontal.is_none() {
          return (None,Some(horizontals[0]));
        }
        if let Some(horizontal) = older_horizontal {
          for elem in horizontals {
            if elem!=horizontal {return (None,Some(elem))}
          }
        }
      }
    }
    (None,None)
  }
  fn copy_with_replaced(&self,i:usize,j:usize) -> Field {
    let mut copy = self.matrix.clone();
    copy[i][j] = if copy[i][j]=='#' {'.'} else {'#'};
    Field {matrix:copy}
  }
}

fn parse(strings:&[String]) -> Vec<Field> {
  let mut fields = vec![];
  let mut field = vec![];
  for line in strings {
    if line.is_empty() {
      fields.push(Field { matrix: field });
      field = vec![];
    } else {
      field.push(line.chars().collect());
    }
  }
  fields.push(Field { matrix: field });
  fields
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(405, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(400, part2(&get_input()));
  }
}