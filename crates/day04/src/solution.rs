use std::vec;

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  total_score(&parsed)
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  scratchcards_duplication(&parsed)
}

struct Scratchcard {
  winning_numbers:Vec<usize>,
  my_numbers:Vec<usize>
}
impl Scratchcard {
  pub fn count_my_winning_numbers(&self) -> u32 {
    let mut score_val:u32 = 0;
    for number in &self.my_numbers {
      if self.winning_numbers.contains(number) {
        score_val+=1;
      }
    }
    score_val
  }
  pub fn score(&self) -> usize {
    let score_val = self.count_my_winning_numbers();
    if score_val>0 {
      usize::pow(2,score_val-1)
    } else {
      0
    }
  }
}

fn total_score(scratchcards:&Vec<Scratchcard>) -> usize {
  scratchcards.iter().map(|s|s.score()).sum()
}

fn scratchcards_duplication(scratchcards:&Vec<Scratchcard>) -> usize {
  let mut scratchcards_total_number = 0;
  let mut scratchcards_count:Vec<usize> = vec![1; scratchcards.len()];
  for (i,scratchcard) in scratchcards.iter().enumerate() {
    let score:usize = scratchcard.count_my_winning_numbers().try_into().unwrap();
    let current_scratchcard_count = scratchcards_count[i];
    for v in i+1..i+1+score {
      scratchcards_count[v] += current_scratchcard_count
    }
    scratchcards_total_number += current_scratchcard_count;
  }
  scratchcards_total_number
}

fn parse(strings:&[String]) -> Vec<Scratchcard> {
  let mut scratchcards = Vec::new();
  for line in strings {
    let game_numbers_split = line.split(":");
    let mut winning_mine_split = game_numbers_split.last().unwrap().split("|");
    let winning_numbers:Vec<usize> = winning_mine_split.nth(0).unwrap().split(' ').filter(|str| str.len()>0).map(|str|str.parse().unwrap()).collect();
    let my_numbers:Vec<usize> = winning_mine_split.last().unwrap().split(' ').filter(|str| str.len()>0).map(|str|str.parse().unwrap()).collect();
    scratchcards.push(Scratchcard{
      winning_numbers,
      my_numbers
    });
  }
  scratchcards
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(13, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(30, part2(&get_input()));
  }
}