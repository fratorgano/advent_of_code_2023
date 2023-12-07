use core::panic;
use std::{vec, collections::HashMap, cmp::Ordering};

pub fn part1(input:&[String]) -> usize {
  let mut parsed = parse(input,false);
  calculate_total_bid(&mut parsed)
}
pub fn part2(input:&[String]) -> usize {
  let mut parsed = parse(input,true);
  calculate_total_bid(&mut parsed)
}

fn calculate_total_bid(hands:&mut Vec<Hand>)->usize {
  hands.sort();
  let mut total_bid = 0;
  for (i,hand) in hands.iter().enumerate() {
    total_bid += hand.bid*(i+1);
  }
  total_bid
}

#[derive(Debug, PartialEq,Eq,PartialOrd,Ord)]
enum HandType {
  HighCard,OnePair,TwoPair,ThreeOfAKind,FullHouse,FourOfAKind,FiveOfAKind
}

#[derive(Debug,Eq)]
struct Hand {
  cards: Vec<usize>,
  bid: usize,
  jokers:bool,
}
impl Hand {
  pub fn find_type(&self)->HandType {
    let hand_type;
    let card_count = self.count_cards();
    if card_count.len()==1 {
      hand_type = HandType::FiveOfAKind;
    } else if card_count.len()==2 {
      let card_number = *card_count.iter().next().unwrap().1;
      if card_number==1 || card_number==4 {
        hand_type = HandType::FourOfAKind;
      } else {
        hand_type = HandType::FullHouse
      }
    } else if card_count.len()==3 {
      let max_card_number = card_count.iter().reduce(|acc,elem| if *elem.1>*acc.1 {elem} else {acc});
      if *max_card_number.unwrap().1==2 {
        hand_type = HandType::TwoPair
      } else {
        hand_type = HandType::ThreeOfAKind
      }
    } else if card_count.len()==4 {
      hand_type = HandType::OnePair
    } else {
      hand_type = HandType::HighCard
    }
    hand_type
  }

  pub fn improve_type(&self) -> HandType {
    let card_count = self.count_cards();
    if let Some(joker_count) = card_count.get(&1) {
      match *joker_count {
        1 => {
          match self.find_type() {
            HandType::HighCard => return HandType::OnePair,
            HandType::OnePair => return HandType::ThreeOfAKind,
            HandType::TwoPair => return HandType::FullHouse,
            HandType::ThreeOfAKind => return HandType::FourOfAKind,
            HandType::FourOfAKind => return HandType::FiveOfAKind,
            _ => panic!("Can't have {:?} while having 1 joker ({:?})",self.find_type(),self)
          }
        },
        2=>{
          match self.find_type() {
            HandType::OnePair => return HandType::ThreeOfAKind,
            HandType::TwoPair => return HandType::FourOfAKind,
            HandType::ThreeOfAKind => return HandType::FiveOfAKind,
            HandType::FullHouse => return HandType::FiveOfAKind,
            _ => panic!("Can't have {:?} while having 2 jokers ({:?})",self.find_type(),self)
          }
        },
        3=>{
          match self.find_type() {
            HandType::ThreeOfAKind => return HandType::FourOfAKind,
            HandType::FullHouse => return HandType::FiveOfAKind,
            _ => panic!("Can't have {:?} while having 3 jokers ({:?})",self.find_type(),self)
        }
        },
        4=>{
          match self.find_type() {
            HandType::FourOfAKind => return HandType::FiveOfAKind,
            _ => panic!("Can't have {:?} while having 4 jokers ({:?})",self.find_type(),self)
        }
        },
        5=>{return HandType::FiveOfAKind},
        _=> panic!("Can't have more than 5 jokers")
      }
    } else {
      return self.find_type()
    }
  }
  fn count_cards(&self) -> HashMap<usize,usize> {
    let mut card_count:HashMap<usize, usize> = HashMap::new();
    for card in &self.cards {
      *card_count.entry(*card).or_insert(0) += 1;
    }
    card_count
  }
}
impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
    let mut count_self = HashMap::new();
    for card in &self.cards {
      *count_self.entry(card).or_insert(0)+=1;
    }
    let mut count_other = HashMap::new();
    for card in &other.cards {
      *count_other.entry(card).or_insert(0)+=1;
    }
    count_self == count_other
  }
}
impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}
impl Ord for Hand {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let self_type;
    let other_type;
    if self.jokers {
      self_type = self.improve_type();
      other_type = other.improve_type();
    } else {
      self_type = self.find_type();
      other_type = other.find_type();
    }
    
    if self_type>other_type {
      Ordering::Greater
    } else if self_type<other_type {
      Ordering::Less
    } else {
      for i in 0..self.cards.len() {
        if self.cards[i]>other.cards[i] {
          return Ordering::Greater
        } else if self.cards[i]<other.cards[i] {
          return Ordering::Less;
        }
      }
      Ordering::Equal
    }
  }
}

fn parse(strings:&[String],jokers:bool) -> Vec<Hand> {
  let mut hands = vec![];
  for line in strings {
    let mut hand_bid_split = line.split(" ");
    let hand_substring = hand_bid_split.next().unwrap();
    let bid = hand_bid_split.next().unwrap().parse::<usize>().unwrap();
    let mut cards = vec![];
    for card_char in hand_substring.chars() {
      let card = match card_char {
        '2'=>2,
        '3'=>3,
        '4'=>4,
        '5'=>5,
        '6'=>6,
        '7'=>7,
        '8'=>8,
        '9'=>9,
        'T'=>10,
        'J'=>if !jokers {11} else {1},
        'Q'=>12,
        'K'=>13,
        'A'=>14,
        _=>panic!("Unexpected type of card found while parsing")
      };
      cards.push(card);
    }
    hands.push(Hand{
      cards,
      bid,
      jokers  
    })
  }
  hands
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(6440, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(5905, part2(&get_input()));
  }
}