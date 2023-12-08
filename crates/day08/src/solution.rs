use std::collections::HashMap;

pub fn part1(input:&[String]) -> usize {
  let (instructions,map) = parse(input);
  follow_instructions(&instructions,&map,"AAA".to_string(),|node|node=="ZZZ")
}
pub fn part2(input:&[String]) -> usize {
  let (instructions,map) = parse(input);
  follow_instructions_simultaneously2(&instructions,&map)
}

fn follow_instructions(instructions:&String,map:&HashMap<String,(String,String)>,
 starting_position:String,ending_condition: fn(&String) -> bool) -> usize {
  let mut current_position = starting_position;
  let mut steps = 0;
  let instructions_char:Vec<char> = instructions.chars().collect();
  // println!("Starting: {}",current_position);
  while !ending_condition(&current_position) {
    let considered_instruction = instructions_char[steps%instructions_char.len()];
    let next_node = map.get(&current_position).unwrap();
    // println!("{}) {}",steps,current_position);
    current_position = match considered_instruction {
        'L' => next_node.0.clone(),
        'R' => next_node.1.clone(),
        _ => panic!("Unexpected direction")
    };
    steps+=1;
  }
  steps
}

fn follow_instructions_simultaneously2(instructions:&String,map:&HashMap<String,(String,String)>) -> usize {
  // randomly tried with least common denominator while trying to wait for the previous solution to end (spoiler:it didn't)
  // and it was the right result, this will work only on very specific inputs:
  // each starting node node reaches exactly one ending one node in its period
  // they all reach it at the same step, no branching/period change
  let current_positions:Vec<String> = map.keys().filter(|x|x.ends_with("A")).map(|s|s.to_string()).collect();
  let periods:Vec<usize> = current_positions.iter().map(
    |pos| follow_instructions(instructions, map, pos.clone(),|pos|pos.ends_with("Z"))
  ).collect();
  lcm(periods)
}

fn gcd(n1:usize,n2:usize) -> usize {
  if n2==0 {return n1};
  return gcd(n2, n1%n2)
}
fn lcm(nums:Vec<usize>) -> usize {
  let mut res = nums[0];
  for i in 1..nums.len()  {
    res = nums[i]*res / gcd(nums[i], res)
  }
  res
}

fn _follow_instructions_simultaneously(instructions:String,map:HashMap<String,(String,String)>) -> usize {
  // Didn't work, too slow
  let mut current_positions:Vec<String> = map.keys().filter(|x|x.ends_with("A")).map(|s|s.to_string()).collect();
  println!("{:?}",current_positions.len());
  let mut steps = 0;
  let instructions_char:Vec<char> = instructions.chars().collect();
  while !current_positions.iter().all(|s|s.ends_with("Z")) {
    let considered_instruction = instructions_char[steps%instructions_char.len()];
    // let next_node = map.get(&current_position).unwrap();
    let next_nodes = current_positions.iter().map(|pos| map.get(pos));
    println!("{}) {:?}",steps,current_positions);
    current_positions = match considered_instruction {
        'L' => next_nodes.map(|node| node.unwrap().0.clone()).collect(),
        'R' => next_nodes.map(|node| node.unwrap().1.clone()).collect(),
        _ => panic!("Unexpected direction")
    };
    steps+=1;
  }
  println!("{}) {:?}",steps,current_positions);
  steps-1
}

fn parse(strings:&[String]) -> (String,HashMap<String,(String,String)>) {
  let mut input_iter = strings.iter();
  // get instructions from first line of input
  let instructions = input_iter.next().unwrap().clone();
  let mut nodes_map = HashMap::new();
  // skip empty line
  input_iter.next();
  for line in input_iter {
    let mut node_lr_split = line.split(" = ");
    let node = node_lr_split.next().unwrap().to_string();
    let mut l_r_split = node_lr_split.next().unwrap().split(", ");
    let left = l_r_split.next().unwrap().replace("(", "");
    let right = l_r_split.next().unwrap().replace(")", "");
    nodes_map.insert(node, (left,right));
  }
  (instructions,nodes_map)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input1() -> Vec<String> {
    "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)".lines().map(|s| String::from(s.trim())).collect()
  }
  fn get_input2() -> Vec<String> {
    "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)".lines().map(|s| String::from(s.trim())).collect()
  }
  fn get_input3() -> Vec<String> {
    "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1_1() {
    assert_eq!(2, part1(&get_input1()));
  }

  #[test]
  fn test_part1_2() {
    assert_eq!(6, part1(&get_input2()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(6, part2(&get_input3()));
  }
}