pub fn part1(input:&[String]) -> i64 {
  let parsed = parse(input);
  let mut total = 0;
  for series in parsed {
    total += predict_next(&series);
  }
  total
}
pub fn part2(input:&[String]) -> i64 {
  let parsed = parse(input);
  let mut total = 0;
  for series in parsed {
    total += predict_previous(&series);
  }
  total
}

pub fn predict_previous(series:&Vec<i64>) -> i64 {
  let mut looping_series = series.clone();
  //println!("{:?}",series);
  let mut firsts = vec![];
  firsts.push(looping_series.first().unwrap().clone());
  while !looping_series.iter().all(|x|*x==0) {
    looping_series = find_differences(&looping_series);
    firsts.push(looping_series.first().unwrap().clone());
  }
  let mut prediction:i64 = 0;
  for number in firsts.iter().rev() {
    prediction = number-prediction; 
  }
  prediction
}

pub fn predict_next(series:&Vec<i64>) -> i64 {
  let mut looping_series = series.clone();
  //println!("{:?}",series);
  let mut lasts = vec![];
  lasts.push(looping_series.last().unwrap().clone());
  while !looping_series.iter().all(|x|*x==0) {
    looping_series = find_differences(&looping_series);
    lasts.push(looping_series.last().unwrap().clone());
    // println!("new : {:?}",looping_series);
  }
  let prediction:i64 = lasts.iter().sum();
  // println!("pred: {:?}",prediction);
  prediction
}

pub fn find_differences(series:&Vec<i64>) -> Vec<i64> {
  let mut differences = vec![];
  for i in 0..series.len()-1 {
    differences.push(series[i+1]-series[i])
  }
  differences
}

fn parse(strings:&[String]) -> Vec<Vec<i64>> {
  strings.iter().map(|line|line.split(" ").map(|num|num.parse::<i64>().unwrap()).collect()).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(114, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(2, part2(&get_input()));
  }
}