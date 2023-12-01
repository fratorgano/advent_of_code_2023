pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  parsed.iter().fold(0, |acc,x|acc+x)
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse2(input);
  parsed.iter().fold(0, |acc,x|acc+x)
}

fn parse(strings:&[String]) -> Vec<usize> {
  let mut parsed = vec![];
  for line in strings.iter()  {
    let mut number_string = String::new();
    for c in line.chars() {
      if c.is_numeric() {
        number_string.push(c);
      }
    }
    // if the string is of length 1, double it
    // if the length is greater, keep only first and last digit
    if number_string.len()==1 {
      number_string.push(number_string.chars().next().unwrap());
    } else {
      let first_digit = number_string.chars().nth(0).unwrap();
      let last_digit = number_string.chars().last().unwrap();
      number_string = String::new();
      number_string.push(first_digit);
      number_string.push(last_digit);
    }
    // println!("{}",number_string);
    parsed.push(number_string.parse::<usize>().unwrap())
  }
  parsed
}

fn parse2(strings:&[String]) -> Vec<usize> {
  let mut parsed = vec![];
  let valid_string_digits = [("one",'1'),("two",'2'),("three",'3'),("four",'4'),("five",'5'),("six",'6'),("seven",'7'),("eight",'8'),("nine",'9')];
  for line in strings.iter()  {
    let mut number_string = String::new();
    for (i,c) in line.chars().enumerate() {
      if c.is_numeric() {
        number_string.push(c);
      } else {
        // if c is numeric, take the next five characters and check if there's a number inside
        for (string_digit,digit) in valid_string_digits  {
            let upper_bound = if i+string_digit.len()>=line.len() {line.len()} else {i+string_digit.len()};
            let substring = &line[i..upper_bound];
            if substring.contains(string_digit) {
              number_string.push(digit);
            }
        }
      }
    }
    // if the string is of length 1, double it
    // if the length is greater, keep only first and last digit
    if number_string.len()==1 {
      number_string.push(number_string.chars().next().unwrap());
    } else {
      let first_digit = number_string.chars().nth(0).unwrap();
      let last_digit = number_string.chars().last().unwrap();
      number_string = String::new();
      number_string.push(first_digit);
      number_string.push(last_digit);
    }
    // println!("{}",number_string);
    parsed.push(number_string.parse::<usize>().unwrap())
  }
  parsed
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet".lines().map(|s| String::from(s.trim())).collect()
  }
  fn get_input2() -> Vec<String> {
    "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(142, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(281, part2(&get_input2()));
  }
}