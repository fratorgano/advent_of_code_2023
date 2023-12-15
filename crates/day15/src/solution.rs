use std::collections::HashMap;

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  parsed.iter().map(|step|hash(&step)).sum()
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  let mut boxes = HashMap::new();
  for step in parsed {
    run_step(&step, &mut boxes)
  }
  calculate_focusing_pover(boxes)
}

fn calculate_focusing_pover(boxes:HashMap<usize,Vec<(String,usize)>>) -> usize {
  let mut total = 0;
  for light_box in boxes {
    for (i,lens) in light_box.1.iter().enumerate() {
      total += (light_box.0+1) * (i+1) * lens.1 
    }
  }
  total
}

fn run_step(step:&String, boxes:&mut HashMap<usize,Vec<(String,usize)>>) {
  let lens_label = extract_label(step);
  let box_id = hash(&lens_label);
  if step.contains("-") {
    // I need to remove the lens from the box boxid
    if let Some(lens_list) = boxes.get_mut(&box_id) {
      if let Some(index) = lens_list.iter().position(|lens| *lens.0 == lens_label) {
       lens_list.remove(index);
      }
    }
  } else {
    // I need to add a lens to the box boxid
    let focal_length:usize = step.chars().filter(|ch|ch.is_numeric()).collect::<String>().parse().unwrap();
    if let Some(lens_list) = boxes.get_mut(&box_id) {
      if let Some(index) = lens_list.iter().position(|lens| *lens.0 == lens_label) {
        // if already present, update
        lens_list[index] = (lens_label,focal_length);
      } else {
        lens_list.push((lens_label,focal_length));
      }
    } else {
      boxes.insert(box_id, vec![(lens_label,focal_length)]);
    }
  }
}

fn extract_label(step:&String) -> String {
  step.chars().filter(|ch| ch.is_alphabetic()).collect::<String>()
}

fn hash(s:&String) -> usize {
  let mut hash:usize = 0;
  for elem in s.as_bytes() {
    hash = (hash+(*elem as usize)) * 17;
    hash = hash % 256
  }
  hash
}

fn parse(strings:&[String]) -> Vec<String> {
  strings[0].split(',').map(|s|s.to_string()).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(1320, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(145, part2(&get_input()));
  }
}