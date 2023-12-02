use core::panic;

#[derive(Debug)]
pub struct Set {
  red_list:Vec<usize>,
  green_list:Vec<usize>,
  blue_list:Vec<usize>,
}

impl Set {
  pub fn is_feasible(&self,red:usize,green:usize,blue:usize)->bool {
    let red_total = self.red_list.iter().fold(0, |acc,n|acc+n);
    let green_total = self.green_list.iter().fold(0, |acc,n|acc+n);
    let blue_total = self.blue_list.iter().fold(0, |acc,n|acc+n);
    red_total<=red && green_total<=green && blue_total<=blue
  }
  pub fn max_cubes(&self)->(usize,usize,usize) {
    let red_max = self.red_list.iter().max().unwrap_or(&0);
    let green_max = self.green_list.iter().max().unwrap_or(&0);
    let blue_max = self.blue_list.iter().max().unwrap_or(&0);
    return (*red_max,*green_max,*blue_max)
  }
}

#[derive(Debug)]
pub struct Game {
  number:usize,
  sets:Vec<Set>
}

impl Game {
  pub fn is_feasible(&self,red:usize,green:usize,blue:usize)->bool {
    for set in &self.sets {
      if !set.is_feasible(red, green, blue) {
        return false
      }
    }
    true
  }
  pub fn power(&self) -> usize{
    let mut red_max=0;
    let mut green_max=0;
    let mut blue_max=0;
    for set in &self.sets {
      let (red,green,blue) = set.max_cubes();
      red_max = red_max.max(red);
      green_max = green_max.max(green);
      blue_max = blue_max.max(blue);
    }
    red_max*green_max*blue_max
  }
}

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  parsed.iter().filter(|game| game.is_feasible(12,13,14)).fold(0, |acc,game|acc+game.number)
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  let mut power = 0;
  for game in parsed {
    power+=game.power();
  }
  power
}

fn parse(strings:&[String]) -> Vec<Game> {
  let mut games = Vec::new();
  for line in strings {
    let game_substring = line.split(": ").last().unwrap();
    let game_number =line.split(": ").nth(0).unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
    let set_substrings = game_substring.split("; ");
    let mut sets = Vec::new();
    for set_string in set_substrings {
      let mut reds = Vec::new();
      let mut greens = Vec::new();
      let mut blues = Vec::new();
      let color_strings = set_string.split(", ");
      for color_string in color_strings {
        let mut parts = color_string.split(" ");
        let number = parts.next().unwrap().parse::<usize>().unwrap();
        let color = parts.next().unwrap();
        match color {
            "red"=> reds.push(number),
            "green"=>greens.push(number),
            "blue"=>blues.push(number),
            _=>panic!("Unexpected input")
        }
      }
      sets.push(Set{
        red_list:reds,
        green_list:greens,
        blue_list:blues
      });
    }
    games.push(Game {number: game_number, sets })
  }
  games
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(8, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(2286, part2(&get_input()));
  }
}