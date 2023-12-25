use std::{vec, collections::{HashSet, HashMap}};

pub fn part1(input:&[String]) -> usize {
  let sorted = parse(input);
  let fallen = simulate_fall(&sorted);
  count_removable(&fallen) as usize
}
pub fn part2(input:&[String]) -> usize {
  let sorted = parse(input);
  find_total_falls(&sorted) as usize
}

pub fn simulate_fall(bricks:&Vec<Brick>) -> HashMap<usize, HashSet<usize>> {
  let max_x = bricks.iter().max_by(|a,b|a.end.x.cmp(&b.end.x)).unwrap().end.x;
  let max_y = bricks.iter().max_by(|a,b|a.end.y.cmp(&b.end.y)).unwrap().end.y;
  let max_z = bricks.iter().max_by(|a,b|a.end.z.cmp(&b.end.z)).unwrap().end.z;
  let mut area:Vec<Vec<Vec<i64>>> = vec![];
  for z in 0..=max_z {
    let mut matrix = vec![];
    for _ in 0..=max_y {
      let mut vector = vec![];
      for _ in 0..=max_x {
        vector.push(if z==0 {0} else {-1});
      }
      matrix.push(vector);
    }
    area.push(matrix);
  }
  let mut can_move = true;
  let mut fallen_bricks = HashMap::new();
  for brick in bricks.iter() {
    let mut new_brick = brick.clone();
    let mut support_set = HashSet::new();
    while can_move {
      let blocks = new_brick.blocks();
      // check for support 
      support_set.clear();
      for b in blocks {
        if area[b.z-1][b.y][b.x]>=0 {
          support_set.insert(area[b.z-1][b.y][b.x] as usize);
        }
      }
      can_move = support_set.len()==0;
      // can_move = blocks.iter().all(|b| area[b.z-1][b.y][b.x]==false);
      if can_move {
        new_brick.start.z -=1;
        new_brick.end.z -=1;
        //println!("{:?}",new_brick);
      }
    }
    can_move = true;
    // brick is in final position, set it in the area and add it to fallen bricks
    let fallen_blocks = new_brick.blocks();
    for block in fallen_blocks {
      area[block.z][block.y][block.x] = new_brick.number as i64;
    }
    /* println!("{:?}",brick);
    println!("{:?}",new_brick);
    println!("{:?}",&support_set);
    println!(); */
    fallen_bricks.insert(new_brick.number,support_set);
  }
  fallen_bricks
}

pub fn simulate_fall_alternative(bricks:&Vec<Brick>) -> (Vec<Brick>,u64) {
  // simulate bricks falling from their initial position until they reach the bottom or they are supported by another brick
  // sort bricks by z coordinate
  let mut sorted_bricks = bricks.clone();
  sorted_bricks.sort_by(|a,b|a.start.z.cmp(&b.start.z));
  let max_x = bricks.iter().max_by(|a,b|a.end.x.cmp(&b.end.x)).unwrap().end.x;
  let max_y = bricks.iter().max_by(|a,b|a.end.y.cmp(&b.end.y)).unwrap().end.y;
  // create a map that stores the max z coordinate for each x,y coordinate
  let mut max_z_map = HashMap::new();
  for x in 0..=max_x {
    for y in 0..=max_y {
      max_z_map.insert((x,y), 1);
    }
  }
  // for each brick, check if it is supported by another brick, if not, move it down until it is supported or it reaches the bottom
  let mut total_falls = 0;
  let mut final_bricks = vec![];
  for brick in sorted_bricks {
    let mut new_brick = brick.clone();
    let mut can_move = true;
    let mut fell = false;
    while can_move {
      // check if a brick is moveable down by checking each of its blocks if it can move down
      // println!("{:?}",new_brick);
      for block in new_brick.blocks() {
        if max_z_map.get(&(block.x,block.y)).unwrap() == &(block.z) {
          can_move = false;
          break;
        }
      }
      // if it can move down, move it down
      if can_move {
        new_brick.start.z -= 1;
        new_brick.end.z -= 1;
        fell = true;
      } else {
        // if it can't move down, set the max z coordinate for each of its blocks and break
        for block in new_brick.blocks() {
          max_z_map.insert((block.x,block.y), block.z+1);
        }
        // println!("{:?}",max_z_map);
        if fell {
          /* println!("fallen: {:?}",brick.number);
          println!("fallen: {:?}",brick); */
          total_falls+=1;
        }
        final_bricks.push(new_brick);
      }
    }
  }
  /* println!("final_bricks: {:?}",final_bricks);
  println!("final: {:?}",max_z_map);
  println!("total_falls: {:?}",total_falls); */
  (final_bricks,total_falls)
}

fn count_removable(fallen_bricks:&HashMap<usize, HashSet<usize>>) -> usize {
  let mut removable_count = 0;
  let mut needed = vec![];
  for i in 0..fallen_bricks.len() {
    let brick_index = i+1;
    let mut is_needed = false;
    for (_brick,supports) in fallen_bricks {
      if supports.len()==1 && supports.contains(&brick_index) {
        is_needed = true;
        break;
      }
    }
    if !is_needed {
      removable_count+=1;
    } else {
      needed.push(i);
    }
  }
  removable_count
}

fn find_total_falls(bricks:&Vec<Brick>) -> u64 {
  // for each brick, remove it and simulate fall, count removable bricks
  let mut total_falls = 0;
  let (fallen_bricks,_) = simulate_fall_alternative(bricks);
  for i in 0..fallen_bricks.len() {
    let mut new_bricks = fallen_bricks.clone();
    new_bricks.remove(i);
    let (_,count) = simulate_fall_alternative(&new_bricks);
    total_falls+=count;
  }
  total_falls
}

#[derive(Debug, Clone, Copy,PartialEq, Eq, Hash)]
pub struct MinHeapEntry(usize,usize);
impl PartialOrd for MinHeapEntry {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    let ord = match self.1.cmp(&other.1) {
      std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
      std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
      std::cmp::Ordering::Greater => std::cmp::Ordering::Less
    };
    Some(ord)
  }
}
impl Ord for MinHeapEntry {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.1.cmp(&other.1)
  }
}

fn parse(strings:&[String]) -> Vec<Brick> {
  let mut bricks = vec![];
  for line in strings {
    let (first,second) = line.split_once("~").unwrap();
    let mut first_vals = first.split(",").map(|s|s.parse::<usize>().unwrap());
    let first_coordinate = Coordinate{x:first_vals.next().unwrap(),y:first_vals.next().unwrap(),z:first_vals.next().unwrap()};
    let mut seconds_vals = second.split(",").map(|s|s.parse::<usize>().unwrap());
    let second_coordinate = Coordinate{x:seconds_vals.next().unwrap(),y:seconds_vals.next().unwrap(),z:seconds_vals.next().unwrap()};
    bricks.push((first_coordinate,second_coordinate));
  }
  bricks.sort_by(|a,b|a.0.z.cmp(&b.0.z));
  let mut sorted_bricks = vec![];
  for (i,brick) in bricks.iter().enumerate() {
    let brick = Brick { start: brick.0, end: brick.1, number: i+1 };
    sorted_bricks.push(brick.clone());
  }
  sorted_bricks
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash )]
pub struct Brick{
  start:Coordinate,
  end:Coordinate,
  number:usize
}
impl Brick {
  pub fn blocks(&self) -> Vec<Coordinate> {
    let mut coordinates = vec![];
    if self.start.eq(&self.end) {
      coordinates.push(self.start);
      return coordinates;
    }
    if self.start.x != self.end.x {
      for x in self.start.x..=self.end.x {
        coordinates.push(Coordinate{x,y:self.start.y,z:self.start.z})
      }
    }
    if self.start.y != self.end.y {
      for y in self.start.y..=self.end.y {
        coordinates.push(Coordinate{x:self.start.x,y,z:self.start.z})
      }
    }
    if self.start.z != self.end.z {
      for z in self.start.z..=self.end.z {
        coordinates.push(Coordinate{x:self.start.x,y:self.start.y,z})
      }
    }
    coordinates
  }
}

#[derive(Debug,Clone,Copy,PartialEq, Eq,Hash)]
pub struct Coordinate {
  x:usize,y:usize,z:usize
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(5, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(7, part2(&get_input()));
  }

  #[test]
  fn test_blocks() {
    let brick = Brick { start: Coordinate { x: 0, y: 0, z: 1 }, end: Coordinate { x: 0, y: 0, z: 1 }, number: 1 };
    let blocks = brick.blocks();
    assert_eq!(1,blocks.len());
    let brick = Brick { start: Coordinate { x: 0, y: 0, z: 1 }, end: Coordinate { x: 0, y: 0, z: 2 }, number: 1 };
    let blocks = brick.blocks();
    assert_eq!(2,blocks.len());
  }
}