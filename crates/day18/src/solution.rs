pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  dig_lake(&parsed)
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse_2(input);
  dig_lake(&parsed)
}

pub fn dig_lake(instructions:&Vec<Instruction>) -> usize {
  let mut vertices = vec![];
  let mut position:(i64,i64) = (0,0);
  let mut len = 0;
  for instruction in instructions.iter() {
    // println!("instr: {:?}",instruction);
    vertices.push(position);
    match instruction.direction {
      Direction::Up => position.1 -= instruction.amount as i64,
      Direction::Down => position.1 += instruction.amount as i64,
      Direction::Left => position.0 -= instruction.amount as i64,
      Direction::Right => position.0 += instruction.amount as i64,
    }
    len+=instruction.amount as usize;
  }
  find_area_2(&vertices) as usize + len/2 +1
}

fn find_area_2(vertices:&Vec<(i64,i64)>) -> u64 {
  let mut area = 0;
  for i in 0..vertices.len() {
    area += vertices[i].0 * vertices[(i+1)%vertices.len()].1 - vertices[(i+1)%vertices.len()].0 * vertices[i].1
  }
  area.unsigned_abs()/2
}

#[derive(Debug)]
pub struct Instruction {
  amount:u64,
  direction:Direction
}

#[derive(Debug, Clone, Copy,PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
  Up,Down,Left,Right
}

fn parse(strings:&[String]) -> Vec<Instruction> {
  let mut instructions = vec![];
  for line in strings {
    let mut split = line.split(" ");
    let direction = split.next().map(|s| {
      match s {
        "R" => Direction::Right,
        "L" => Direction::Left,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("Unexpected direction")
      }
    }).unwrap();
    let amount = split.next().unwrap().parse::<u64>().unwrap();
    instructions.push(Instruction { amount, direction})
  }
  instructions
}

fn parse_2(strings:&[String]) -> Vec<Instruction> {
  let mut instructions = vec![];
  for line in strings {
    let mut split = line.split(" ");
    let color:Vec<char> = split.nth(2).unwrap().chars().filter(|x|x.is_alphanumeric()).collect();
    let direction = match color[5] {
      '0' => Direction::Right,
      '1' => Direction::Down,
      '2' => Direction::Left,
      '3' => Direction::Up,
        _ => panic!("Unexpected direction")
    };
    instructions.push(Instruction { amount: hex_string_to_num(&color[0..5]), direction})
  }
  instructions
}

fn hex_string_to_num(s:&[char]) -> u64 {
  let mut value:u64 = 0;
  for (i,c) in s.iter().rev().enumerate() {
    value += 16_u64.pow(i.try_into().unwrap()) * c.to_digit(16).unwrap() as u64
  }
  value
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(62, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(952408144115, part2(&get_input()));
  }
}