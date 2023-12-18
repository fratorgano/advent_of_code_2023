use std::collections::{HashMap, BinaryHeap};

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  // print_matrix(&parsed);
  a_star(Point{x:0,y:0},&parsed,3,1)
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  a_star(Point{x:0,y:0},&parsed,10,4)
}

fn a_star(start:Point,grid:&Vec<Vec<usize>>,max_direction:usize,turning_time:usize) -> usize {
  let goal = Point{x:grid[0].len()-1,y:grid.len()-1};

  let mut open_set = BinaryHeap::new();
  open_set.push(MinHeapEntry{
    node:Node {
      point:start,
      direction:Direction::Right,
      dir_amount:1
    },
    heuristic_val:start.manhattan_distance(goal),
  });
  open_set.push(MinHeapEntry{
    node:Node {
      point:start,
      direction:Direction::Down,
      dir_amount:1
    },
    heuristic_val:start.manhattan_distance(goal),
  });

  // let mut came_from = HashMap::new();
  let mut g_score = HashMap::new();
  // set starting elements cost to 0
  for elem in open_set.clone() {
    g_score.insert(elem.node, 0 as usize);
  }
  // set starting costs for starting elements using an heuristic (manhattan distance)
  let mut last = None;
  while !open_set.is_empty() {
    let heap_entry = open_set.pop().unwrap();
    if heap_entry.node.point.eq(&goal) && heap_entry.node.dir_amount>=turning_time {
      last = Some(heap_entry.node);
      // println!("GOAL REACHED");
      break;
    }
    for neighbour in heap_entry.node.neighbours((goal.x,goal.y),max_direction,turning_time) {
      let tentative_score = g_score[&heap_entry.node] + grid[neighbour.point.y][neighbour.point.x];
      let prev_score = g_score.get(&neighbour).unwrap_or(&usize::MAX);
      // println!("{:?} -> {:?} ({})",neighbour,tentative_score,prev_score);
      
      if tentative_score<*prev_score {
        // println!("updating score");
        // came_from.insert(neighbour, heap_entry.node);
        g_score.insert(neighbour, tentative_score);
        // todo: update cost in open_set
        if open_set.iter().filter(|e|e.node.eq(&neighbour)).count()==0 {
          open_set.push(MinHeapEntry { node: neighbour, heuristic_val: g_score[&heap_entry.node] + neighbour.point.manhattan_distance(goal) });
        }
      }
    }
  }
  /* let mut path = vec![];
  if let Some(last_node) = last {
    let mut current = last_node;
    loop {
      path.push(current.point);
      if let Some(prev) = came_from.get(&current) {
        current = *prev;
      } else {
        path.reverse();
        break
      }
      // println!("{:?}",current.point);
    }
  }

  let mut path_matrix = grid.clone();
  for (i,elem) in path.iter().enumerate() {
    path_matrix[elem.y][elem.x] = 0;
  }
  print_matrix(&path_matrix); */

  if let Some(last_node) = last {
    *g_score.get(&last_node).unwrap()
  } else {
    0
  }
}
#[derive(Debug, Clone, Copy,PartialEq, Eq, Hash)]
pub struct Node {
  point:Point,
  direction:Direction,
  dir_amount:usize
}
impl Node {
  pub fn neighbours(&self, limits:(usize,usize),max_direction:usize,turning_time:usize) -> Vec<Node> {
    let mut neighs = vec![];
    if self.dir_amount<turning_time {
      // can only go in the same direction 
      match self.direction {
        Direction::Left => {if self.point.x>0{neighs.push(Node{
          point:Point{x:self.point.x-1,y:self.point.y},
          direction:Direction::Left,
          dir_amount:self.dir_amount+1
        })}},
        Direction::Right => {if self.point.x<limits.0{neighs.push(Node{
          point:Point{x:self.point.x+1,y:self.point.y},
          direction:Direction::Right,
          dir_amount:self.dir_amount+1
        })}},
        Direction::Up => {if self.point.y>0 {neighs.push(Node{
          point:Point{x:self.point.x,y:self.point.y-1},
          direction:Direction::Up,
          dir_amount:self.dir_amount+1
        })}},
        Direction::Down => {if self.point.y<limits.1{neighs.push(Node{
          point:Point{x:self.point.x,y:self.point.y+1},
          direction:Direction::Down,
          dir_amount:self.dir_amount+1
        })}},
      }
      return neighs;
    }
    // move left
    if self.point.x>0 && self.direction!=Direction::Right {
      let dir_amount = if Direction::Left == self.direction {self.dir_amount+1} else {1};
      if dir_amount<=max_direction {
        neighs.push(Node{
          point:Point{x:self.point.x-1,y:self.point.y},
          direction:Direction::Left,
          dir_amount
        })
      }
    }
    // move right
    if self.point.x<limits.0 && self.direction!=Direction::Left {
      let dir_amount = if Direction::Right == self.direction {self.dir_amount+1} else {1};
      if dir_amount<=max_direction {
        neighs.push(Node{
          point:Point{x:self.point.x+1,y:self.point.y},
          direction:Direction::Right,
          dir_amount
        })
      } 
    }
    // move up
    if self.point.y>0 && self.direction!=Direction::Down {
      let dir_amount = if Direction::Up == self.direction {self.dir_amount+1} else {1};
      if dir_amount<=max_direction {
        neighs.push(Node{
          point:Point{x:self.point.x,y:self.point.y-1},
          direction:Direction::Up,
          dir_amount
        })
      }
    }
    // move down
    if self.point.y<limits.1 && self.direction!=Direction::Up {
      let dir_amount = if Direction::Down == self.direction {self.dir_amount+1} else {1};
      if dir_amount<=max_direction {
        neighs.push(Node{
          point:Point{x:self.point.x,y:self.point.y+1},
          direction:Direction::Down,
          dir_amount
        })
      }
    }
    neighs
  }
}
#[derive(Debug, Clone, Copy,PartialEq, Eq, Hash)]
pub struct MinHeapEntry {
  node:Node,
  heuristic_val: usize,
}
impl PartialOrd for MinHeapEntry {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    let ord = match self.heuristic_val.cmp(&other.heuristic_val) {
      std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
      std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
      std::cmp::Ordering::Greater => std::cmp::Ordering::Less
    };
    Some(ord)
  }
}
impl Ord for MinHeapEntry {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.heuristic_val.cmp(&other.heuristic_val)
  }
}

#[derive(Debug, Clone, Copy,PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
  Up,Down,Left,Right
}

#[derive(Debug,Clone, Copy,PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
  x:usize,
  y:usize
}
impl Point {
  pub fn manhattan_distance(&self,goal:Point) -> usize {
    ((self.x as i64).abs_diff(goal.x as i64) + (self.y as i64).abs_diff(goal.y as i64)) as usize
  }
}

fn parse(strings:&[String]) -> Vec<Vec<usize>> {
  let mut matrix = vec![];
  for line in strings {
    let mut row = vec![];
    for elem in line.chars() {
      row.push(elem.to_digit(10).unwrap() as usize)
    }
    matrix.push(row);
  }
  matrix
}

/* fn print_matrix(matrix: &Vec<Vec<usize>>) {
  for line in matrix {
    for elem in line {
      print!("{} ",elem)
    }
    println!();
  }
  println!();
} */

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533".lines().map(|s| String::from(s.trim())).collect()
  }

  fn get_input_2() -> Vec<String> {
    "111111111111
    999999999991
    999999999991
    999999999991
    999999999991".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(102, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(94, part2(&get_input()));
  }

  #[test]
  fn test_part2_2() {
    assert_eq!(71, part2(&get_input_2()));
  }

  #[test]
  fn test_manhattan() {
    assert_eq!(5, Point{x:2,y:9}.manhattan_distance(Point{x:3,y:5}));
  }
}