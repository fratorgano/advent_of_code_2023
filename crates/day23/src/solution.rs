use std::collections::{HashMap, VecDeque};

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  let compressed = compress_matrix(&parsed,neighbours_p1,true);
  let start = (1,0);
  let goal = (parsed[0].len()-2,parsed.len()-1);
  find_max_path(start,goal,compressed)
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  let compressed = compress_matrix(&parsed,neighbours_p2,false);
  let start = (1,0);
  let goal = (parsed[0].len()-2,parsed.len()-1);
  find_max_path(start,goal,compressed)
}

pub fn find_max_path(start:(usize,usize),goal:(usize,usize),matrix:HashMap<(usize, usize), Vec<((usize, usize), usize)>>) -> usize {
  let mut max = 0;
  let mut queue = VecDeque::new();
  queue.push_back((start,0,vec![]));
  while let Some((node,cost,mut visited)) = queue.pop_front() {
    if node == goal {
      // println!("{}",max);
      if cost>max {
        max = cost;
      }
      continue;
    }
    visited.push(node);
    let neighs = matrix.get(&node).unwrap();
    for (neigh,neigh_cost) in neighs {
      if !visited.contains(neigh) {
        queue.push_back((*neigh, cost+neigh_cost,visited.clone()));
      }
    }
  }
  max
}

pub fn compress_matrix(matrix:&Vec<Vec<char>>,
  neighbor_function: fn(position:(usize,usize),matrix:&Vec<Vec<char>>,previous:Option<(usize,usize)>,) -> Vec<(usize,usize)>,
  is_part1:bool
) -> HashMap<(usize,usize),Vec<((usize,usize),usize)>> {
  let start = (1,0);
  let goal = (matrix[0].len()-2,matrix.len()-1);
  let mut nodes = vec![];
  nodes.push(start);
  nodes.push(goal);
  for y in 0..matrix.len() {
    for x in 0..matrix[0].len() {
      if matrix[y][x] != '#' {
        let neighbours = neighbor_function((x,y),&matrix,None);
        if is_part1 {
          // Part 2: a node is a node in the compressed graph if it has at least 2 ways to leave
          let out_count = count_ways_to_leave((x,y),matrix);
          if out_count>=2 {
            nodes.push((x,y))
          }
        } else {
          // Part 2: a node is a node in the compressed graph if it has at least 3 neighbours 
          if neighbours.len()>=3 {
            nodes.push((x,y))
          }
        }
      }
    }
  }
  let mut edges:HashMap<(usize,usize),Vec<((usize,usize),usize)>> = HashMap::new();
  for node in nodes.iter() {
    for mut neighbor in neighbor_function(*node, matrix,None) {
      // track previous node
      let mut prev = *node;
      let mut dist = 0;
      // walk until we hit another node
      loop {
        dist+=1;
        let neighbours_vec = neighbor_function(neighbor, matrix,Some(prev));
        if neighbours_vec.len() != 1 {
          edges.entry(*node).or_default().push((neighbor,dist));
          break;
        }
        prev = neighbor;
        neighbor = neighbours_vec[0];
      }
    }
  }
  edges
}

pub fn count_ways_to_leave(position:(usize,usize),matrix:&Vec<Vec<char>>) -> u64 {
  let limits = (matrix[0].len(),matrix.len());
  let mut ways = 0;
  if position.0 > 0 && matrix[position.1][position.0-1]=='<'  {
    ways+=1;
  }
  if position.0+1<limits.0 && matrix[position.1][position.0+1]=='>' {
    ways+=1;
  }
  if position.1 > 0 && matrix[position.1-1][position.0]=='^'  {
    ways+=1;
  }
  if position.1+1<limits.1 && matrix[position.1+1][position.0]=='v' {
    ways+=1;
  }
  ways
}

pub fn neighbours_p1(position:(usize,usize),matrix:&Vec<Vec<char>>,previous:Option<(usize,usize)>) -> Vec<(usize, usize)> {
  let limits = (matrix[0].len(),matrix.len());
  let mut neighs = vec![];
  if position.0 > 0 && (matrix[position.1][position.0-1]=='.' || matrix[position.1][position.0-1]=='<')  {
    neighs.push((position.0-1,position.1));
  }
  if position.0+1<limits.0 && (matrix[position.1][position.0+1]=='.' || matrix[position.1][position.0+1]=='>') {
    neighs.push((position.0+1,position.1))
  }
  if position.1 > 0 && (matrix[position.1-1][position.0]=='.' || matrix[position.1-1][position.0]=='^')  {
    neighs.push((position.0,position.1-1));
  }
  if position.1+1<limits.1 &&(matrix[position.1+1][position.0]=='.' || matrix[position.1+1][position.0]=='v') {
    neighs.push((position.0,position.1+1))
  }
  if let Some(previous) = previous {
    let index = neighs.iter().position(|x| *x == previous);
    if let Some(index) = index {neighs.remove(index);}
  }
  neighs
}

pub fn neighbours_p2(position:(usize,usize),matrix:&Vec<Vec<char>>,previous:Option<(usize,usize)>) -> Vec<(usize,usize)> {
  let limits = (matrix[0].len(),matrix.len());
  let mut neighs = vec![];
  if position.0 > 0 && matrix[position.1][position.0-1]!='#'  {
    neighs.push((position.0-1,position.1));
  }
  // go right
  if position.0+1<limits.0 && matrix[position.1][position.0+1]!='#' {
    neighs.push((position.0+1,position.1))
  }
  if position.1 > 0 && matrix[position.1-1][position.0]!='#'  {
    neighs.push((position.0,position.1-1));
  }
  if position.1+1<limits.1 && matrix[position.1+1][position.0]!='#' {
    neighs.push((position.0,position.1+1))
  }
  if let Some(previous) = previous {
    let index = neighs.iter().position(|x| *x == previous);
    if let Some(index) = index {neighs.remove(index);}
  }
  neighs
}

fn parse(strings:&[String]) -> Vec<Vec<char>> {
  let mut matrix = vec![];
  for line in strings {
    let mut row = vec![];
    for elem in line.chars(){
      row.push(elem)
    }
    matrix.push(row);
  }
  matrix
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "#.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(94, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(154, part2(&get_input()));
  }
}