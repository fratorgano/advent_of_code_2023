use std::collections::HashSet;

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  count_energized(EnergyBeam{position:(0,0),direction:Direction::East},&parsed) 
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  maximize_energized(&parsed)  
}

pub fn maximize_energized(cave:&Vec<Vec<char>>) -> usize {
  let mut max = 0;
  let starting_beams = generate_starting_beams(cave);
  for beam in starting_beams {
    let energized = count_energized(beam, cave);
    if energized>max {
      max=energized;
    }
  }
  max
}
pub fn generate_starting_beams(cave:&Vec<Vec<char>>)-> Vec<EnergyBeam> {
  let mut energy_beams = vec![];
  // west and west
  for i in 0..cave.len() {
    energy_beams.push(EnergyBeam{position:(0,i),direction:Direction::East});
    energy_beams.push(EnergyBeam{position:(cave[0].len()-1,i),direction:Direction::West});
  }
  // north and south
  for i in 0..cave[0].len() {
    energy_beams.push(EnergyBeam{position:(i,0),direction:Direction::South});
    energy_beams.push(EnergyBeam{position:(i,cave.len()-1),direction:Direction::North});
  }
  energy_beams
}

pub fn count_energized(start_beam:EnergyBeam,cave:&Vec<Vec<char>>) -> usize {
  let mut energy_beams = vec![];
  energy_beams.push(start_beam);
  let mut marked_cave = cave.clone();
  let mut new_beams = vec![];
  let mut moves = HashSet::new();
  while energy_beams.len()>0 {
    for beam in energy_beams {
      if !moves.insert(beam.clone()) {
        continue;
      }
      //println!("Considering: {:?}",beam);
      let mut next = beam.next(cave);
      marked_cave[beam.position.1][beam.position.0] = '#';
      //println!("Produced: {:?}",next);
      new_beams.append(&mut next);
    }
    energy_beams = new_beams;
    new_beams = vec![];
  }
  count_energized_matrix(&marked_cave)
}

pub fn count_energized_matrix(matrix:&Vec<Vec<char>>) -> usize {
  let mut sum = 0;
  for row in matrix {
    for elem in row {
      if *elem=='#'{
        sum+=1;
      }
    }
  }
  sum
}

#[derive(Debug,PartialEq,Eq,Hash,Clone, Copy)]
pub struct EnergyBeam {
  position:(usize,usize),
  direction: Direction
}
impl EnergyBeam {
  pub fn next(&self,cave:&Vec<Vec<char>>) -> Vec<EnergyBeam> {
    let elem = cave[self.position.1][self.position.0];
    let max_lens = (cave[0].len(),cave.len());
    let mut new_beams = vec![];
    //println!("Found: {:?} going {:?}",elem,self.direction);
    match (elem, &self.direction) {
      ('|',dir) if dir.eq(&Direction::East)||dir.eq(&Direction::West)  => {
        // beam starts going north and south
        if let Some(pos) = generate_next_position(self.position, &Direction::South, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::South})
        }
        if let Some(pos) = generate_next_position(self.position, &Direction::North, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::North})
        }
      },
      ('-',dir) if dir.eq(&Direction::North)||dir.eq(&Direction::South)  => {
        // beam starts going east and west
        if let Some(pos) = generate_next_position(self.position, &Direction::West, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::West})
        }
        if let Some(pos) = generate_next_position(self.position, &Direction::East, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::East})
        }
      },
      ('\\',Direction::East) => {
        // beam goes south
        if let Some(pos) = generate_next_position(self.position, &Direction::South, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::South})
        }
      },
      ('\\',Direction::West) => {
        // beam goes north
        if let Some(pos) = generate_next_position(self.position, &Direction::North, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::North})
        }
      },
      ('\\',Direction::North) => {
        // beam goes west
        if let Some(pos) = generate_next_position(self.position, &Direction::West, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::West})
        }
      },
      ('\\',Direction::South) => {
        // beam goes east
        if let Some(pos) = generate_next_position(self.position, &Direction::East, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::East})
        }
      },
      ('/',Direction::South) => {
        // beam goes west
        if let Some(pos) = generate_next_position(self.position, &Direction::West, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::West})
        }
      },
      ('/',Direction::North) => {
        // beam goes east
        if let Some(pos) = generate_next_position(self.position, &Direction::East, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::East})
        }
      },
      ('/',Direction::East) => {
        // beam goes north
        if let Some(pos) = generate_next_position(self.position, &Direction::North, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::North})
        }
      },
      ('/',Direction::West) => {
        // beam goes south  
        if let Some(pos) = generate_next_position(self.position, &Direction::South, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:Direction::South})
        }
      },
      _ => {
        if let Some(pos) = generate_next_position(self.position, &self.direction, max_lens) {
          new_beams.push(EnergyBeam{position:pos,direction:self.direction})
        }
      },
    }
    new_beams
  }
}
pub fn generate_next_position(start:(usize,usize),direction:&Direction,limits:(usize,usize)) -> Option<(usize,usize)> {
  match direction {
    Direction::North => {
      if start.1==0 {
        None
      } else {
        Some((start.0,start.1-1))
      }
    },
    Direction::South => {
      if start.1+1>=limits.1 {
        None
      } else {
        Some((start.0,start.1+1))
      }
    },
    Direction::West => {
      if start.0==0 {
        None
      } else {
        Some((start.0-1,start.1))
      }
    },
    Direction::East => {
      if start.0+1>=limits.1 {
        None
      } else {
        Some((start.0+1,start.1))
      }
    }
  }
}

#[derive(Debug,Clone, Copy, PartialEq, Eq,Hash)]
pub enum Direction {
  North,South,East,West
}

fn parse(strings:&[String]) -> Vec<Vec<char>> {
  strings.iter().map(|s|s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    r".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(46, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(51, part2(&get_input()));
  }
}