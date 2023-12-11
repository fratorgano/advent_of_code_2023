pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  find_loop(parsed.0,parsed.1).len()/2
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  let pipe_loop = find_loop(parsed.0,parsed.1.clone());
  find_area(pipe_loop,parsed.1)
}

fn find_area(pipe_loop: Vec<(usize, usize)>, map: Vec<Vec<char>>) -> usize {
  let mut new_map = vec![];
  for i in 0..map.len() {
    let mut new_row = vec![];
    for j in 0..map[0].len() {
      if pipe_loop.contains(&(i,j)) {
        new_row.push(map[i][j])
      } else {
        new_row.push('.')
      }
    }
    new_map.push(new_row);
  }
  let mut area = 0;
  for i in 0..new_map.len() {
    let mut i_j_l_count = 0;
    for j in 0..new_map[0].len() {
      if new_map[i][j]=='.' && i_j_l_count%2==1{
        area+=1;
      } else {
        if new_map[i][j] == '|' || new_map[i][j] == 'J' ||new_map[i][j] == 'L' {
          i_j_l_count+=1;
        }
      }
    }
  }
  area
}

fn find_loop(start:(usize,usize), map: Vec<Vec<char>>) -> Vec<(usize, usize)> {
  let map_size = (map.len(),map[0].len());
  let start_neighbours = find_neighbours(start,map[start.0][start.1], map_size);
  let start_valid_neighbours = validate_neighbours(start, start_neighbours, &map);
  let mut pos = start_valid_neighbours[0];
  let mut previous = start;
  let mut pipe_loop = vec![start];
  while pos!=start {
    pipe_loop.push(pos);
    let neighbours = find_neighbours(pos,map[pos.0][pos.1], map_size);
    let mut valid_neighbours = validate_neighbours(pos, neighbours, &map);
    valid_neighbours.retain(|p|*p!=previous);
    // check if neighbour is valid
    previous = pos;
    pos = *valid_neighbours.first().unwrap();
  }
  pipe_loop
}
/* fn step(position:,positions:&mut HashSet<(usize,usize)>) -> {

} */

fn find_neighbours(position:(usize,usize),pipe_type:char, map_size:(usize,usize)) -> Vec<(usize,usize)> {
  let mut neighbours = vec![];
  match pipe_type {
    'S' => {
      if position.0>0 { // north
        neighbours.push((position.0-1,position.1));
      }
      if position.0<map_size.0 { // south
        neighbours.push((position.0+1,position.1));
      }
      if position.1>0 { // west
        neighbours.push((position.0,position.1-1));
      }
      if position.1<map_size.1 { // east
        neighbours.push((position.0,position.1+1));
      }
    },
    '|' => {
      if position.0>0 { // north
        neighbours.push((position.0-1,position.1));
      }
      if position.0<map_size.0 { // south
        neighbours.push((position.0+1,position.1));
      }
    },
    '-' => {
      if position.1>0 { // west
        neighbours.push((position.0,position.1-1));
      }
      if position.1<map_size.1 { // east
        neighbours.push((position.0,position.1+1));
      }
    },
    'L' => {
      if position.0>0 { // north
        neighbours.push((position.0-1,position.1));
      }
      if position.1<map_size.1 { // east
        neighbours.push((position.0,position.1+1));
      }
    },
    'J' => {
      if position.0>0 { // north
        neighbours.push((position.0-1,position.1));
      }
      if position.1>0 { // west
        neighbours.push((position.0,position.1-1));
      }
    },
    '7' => {
      if position.0<map_size.0 { // south
        neighbours.push((position.0+1,position.1));
      }
      if position.1>0 { // west
        neighbours.push((position.0,position.1-1));
      }
    },
    'F' => {
      if position.0<map_size.0 { // south
        neighbours.push((position.0+1,position.1));
      }
      if position.1<map_size.1 { // east
        neighbours.push((position.0,position.1+1));
      }
    }
    '.' => {},
    _ => {panic!("Unrecognized pipe type")}
  }
  neighbours
}

fn validate_neighbours(pos:(usize,usize), neighbours: Vec<(usize,usize)>, map:&Vec<Vec<char>>) ->  Vec<(usize,usize)> {
  let map_size = (map.len(),map[0].len());
  let mut valid_neighbours = vec![];
  for neighbour in neighbours {
    let neighbour_neigbours = find_neighbours(neighbour, map[neighbour.0][neighbour.1], map_size);
    if neighbour_neigbours.contains(&pos) {
      valid_neighbours.push(neighbour);
    }
  }
  valid_neighbours
}

fn parse(strings:&[String]) -> ((usize, usize), Vec<Vec<char>>) {
  let mut matrix = vec![];
  let mut starting = (0,0);
  for (i,line) in strings.iter().enumerate() {
    let mut new_line = vec![];
    for (j,elem) in line.chars().enumerate() {
      new_line.push(elem);
      if elem=='S' {
        starting = (i,j);
      }
    }
    matrix.push(new_line)
  }
  (starting,matrix)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input_1() -> Vec<String> {
    "-L|F7
    7S-7|
    L|7||
    -L-J|
    L|-JF".lines().map(|s| String::from(s.trim())).collect()
  }

  fn get_input_2() -> Vec<String> {
    "7-F7-
    .FJ|7
    SJLL7
    |F--J
    LJ.LJ".lines().map(|s| String::from(s.trim())).collect()
  }

  fn get_input_3() -> Vec<String> {
    "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1_1() {
    assert_eq!(4, part1(&get_input_1()));
  }

  #[test]
  fn test_part1_2() {
    assert_eq!(8, part1(&get_input_2()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(4, part2(&get_input_3()));
  }
}