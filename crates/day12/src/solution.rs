use std::{usize, collections::HashMap};

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input,1);
  let mut total = 0;
  for (row_string,broken_rows) in parsed {
    let mut cache = HashMap::new();
    let val = possible_sol_rec(row_string.clone(), None, &broken_rows, &mut cache);
    total += val;
  }
  total
}

pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input,5);
  let mut total = 0;
  for (row_string,broken_rows) in parsed {
    let mut cache = HashMap::new();
    let val = possible_sol_rec(row_string.clone(), None, &broken_rows, &mut cache);
    total += val;
  }
  total
}

pub fn possible_sol_rec(s:String, in_group:Option<usize>, rem_groups:&[usize], cache: &mut HashMap<(usize, usize, usize), usize>) ->usize {
  // last element 
  if s.is_empty() {
    // I'm not in a group and there are no more groups
    if in_group.is_none() && rem_groups.is_empty() {
      return 1
    }
    // in all other cases return 0
    return 0
  }
  // If I'm in a group and there are no more groups available, return 0
  if in_group.is_some() && rem_groups.is_empty() {
    return 0;
  }
  // check if the map contains a solution already
  let key = (s.len(),in_group.unwrap_or(0),rem_groups.len());
  if let Some(ways) = cache.get(&key){
    return *ways
  }

  let ways = match (s.chars().nth(0).unwrap(),in_group) {
    ('.',Some(x)) if x!=rem_groups[0] => 0,
    ('.',Some(_)) => possible_sol_rec(remove_first(&s), None, &rem_groups[1..], cache),
    ('.',None) => possible_sol_rec(remove_first(&s), None, rem_groups, cache),
    ('#',Some(x)) => possible_sol_rec(remove_first(&s), Some(x+1), rem_groups, cache),
    ('#',None) => possible_sol_rec(remove_first(&s), Some(1), rem_groups, cache),
    ('?',Some(x)) => {
      // count possibility by continuing with group
      let mut possible_sol = possible_sol_rec(remove_first(&s), Some(x+1), rem_groups, cache);
      // add possibilities of closing group if possible
      if x == rem_groups[0] {
        possible_sol += possible_sol_rec(remove_first(&s),None,&rem_groups[1..],cache)
      }
      possible_sol
    },
    ('?',None) => {
      possible_sol_rec(remove_first(&s), Some(1), rem_groups, cache) + possible_sol_rec(remove_first(&s),None,rem_groups,cache)
    },
    _=>panic!("Unexpected case")
  };
  cache.insert(key, ways);
  ways
}

pub fn remove_first(s:&String) -> String{
  let mut copy = s.clone(); 
  copy.remove(0);
  return copy
}

fn parse(strings:&[String],times:usize) -> Vec<(String,Vec<usize>)> {
  let mut springs_rows = vec![];
  for line in strings {
    let mut row_string_broken_rows_split = line.split(" ");
    let row_string = row_string_broken_rows_split.next().unwrap().to_string();
    let broken_rows:Vec<usize> = row_string_broken_rows_split.next().unwrap().split(",").map(|x|x.parse::<usize>().unwrap()).collect();
    let mut row_string_joined = join_strings((0..times).map(|_|row_string.clone()).collect());
    // simplifies cases for DP
    row_string_joined.push('.');
    springs_rows.push((
      row_string_joined,
      (0..times).flat_map(|_|&broken_rows).copied().collect()
    ));
  }
  springs_rows
}

fn join_strings(strings: Vec<String>) -> String {
  let mut new_str = String::new();
  for (i,s) in strings.iter().enumerate() {
    if i<strings.len()-1 {
      new_str.push_str(&(s.to_owned()+"?"));
    } else {
      new_str.push_str(&(s.to_owned()));
    }
  }
  new_str
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(21, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(525152, part2(&get_input()));
  }
}