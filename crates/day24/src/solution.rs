use ndarray::prelude::*;
use ndarray_linalg::Solve;

pub fn part1(input:&[String]) -> usize {
  let parsed = parse(input);
  count_intersections(&parsed,(200000000000000.0,400000000000000.0)) as usize
}
pub fn part2(input:&[String]) -> usize {
  let parsed = parse(input);
  solve_lin_alg(&parsed)
}

pub fn solve_lin_alg(parsed:&Vec<Hail>) -> usize {
  let p0 = &parsed[0].position;
  let p1 = &parsed[1].position;
  let p2 = &parsed[2].position;
  let v0 = &parsed[0].velocity;
  let v1 = &parsed[1].velocity;
  let v2 = &parsed[2].velocity;

  let b: Array1<f64> = array![
    (p0.y * v0.x - p1.y * v1.x) - (p0.x * v0.y - p1.x * v1.y),
    (p0.y * v0.x - p2.y * v2.x) - (p0.x * v0.y - p2.x * v2.y),
    (p0.z * v0.x - p1.z * v1.x) - (p0.x * v0.z - p1.x * v1.z),
    (p0.z * v0.x - p2.z * v2.x) - (p0.x * v0.z - p2.x * v2.z),
    (p0.z * v0.y - p1.z * v1.y) - (p0.y * v0.z - p1.y * v1.z),
    (p0.z * v0.y - p2.z * v2.y) - (p0.y * v0.z - p2.y * v2.z),
  ];

  let a = array![
    [v1.y - v0.y, v0.x - v1.x, 0.0, p0.y - p1.y, p1.x - p0.x, 0.0],
    [v2.y - v0.y, v0.x - v2.x, 0.0, p0.y - p2.y, p2.x - p0.x, 0.0],
    [v1.z - v0.z, 0.0, v0.x - v1.x, p0.z - p1.z, 0.0, p1.x - p0.x],
    [v2.z - v0.z, 0.0, v0.x - v2.x, p0.z - p2.z, 0.0, p2.x - p0.x],
    [0.0, v1.z - v0.z, v0.y - v1.y, 0.0, p0.z - p1.z, p1.y - p0.y],
    [0.0, v2.z - v0.z, v0.y - v2.y, 0.0, p0.z - p2.z, p2.y - p0.y],
  ];

  let r = a.solve_into(b).unwrap();
  (r[0] + r[1] + r[2]) as usize
}

pub fn count_intersections(hails:&Vec<Hail>, range:(f64,f64)) -> u64 {
  let mut count = 0;
  for i in 0..hails.len() {
    for j in i+1..hails.len() {
      if let Some(intersection) = hails[i].intersection(&hails[j]) {
        match intersection {
          Intersection::Point(p) => {
            if p.x>=range.0 && p.x<=range.1 && p.y>=range.0 && p.y<=range.1 && !hails[i].in_past(&p) && !hails[j].in_past(&p)  {
              count+=1
            }
          },
          Intersection::All => count+=1,
        }
      }
    }
  }
  count
}

#[derive(Debug, Clone, Copy)]
pub struct Hail {
  position:Coordinate3D,
  velocity:Coordinate3D
}
pub enum Intersection {
  Point(Coordinate3D),
  All,
}

impl Hail {
  pub fn intersection(&self,other:&Hail) -> Option<Intersection> {
    let slope_self = self.velocity.y/self.velocity.x;
    let slope_other = other.velocity.y/other.velocity.x;
    let intercept_self = self.position.y - slope_self * self.position.x;
    let intercept_other = other.position.y - slope_other * other.position.x;

    if slope_self == slope_other && intercept_self == intercept_other {
      return Some(Intersection::All);
    } else if slope_self == slope_other {
      return None;
    }

    let x = (intercept_other - intercept_self) / (slope_self - slope_other);
    let y = slope_self * x + intercept_self;

    Some(Intersection::Point(Coordinate3D {x,y,z:0.0}))
  } 

  fn in_past(&self,point:&Coordinate3D) -> bool {
    let x = point.x - self.position.x;
    let y = point.y - self.position.y;

    let x = x / self.velocity.x;
    let y = y / self.velocity.y;

    x < 0.0 && y < 0.0
  }
}
#[derive(Debug, Clone, Copy)]
pub struct Coordinate3D {
  x:f64,
  y:f64,
  z:f64
}

fn parse(strings:&[String]) -> Vec<Hail> {
  let mut hails = vec![];
  for line in strings {
    if let Some((position,velocity)) = line.split_once(" @ ") {
      let position:Vec<f64> = position.split(", ").map(|s|s.parse::<f64>().unwrap()).collect();
      let velocity:Vec<f64> = velocity.split(", ").map(|s|s.trim()).map(|s|s.parse::<f64>().unwrap()).collect();
      hails.push(Hail { 
        position: Coordinate3D {x:position[0],y:position[1],z:position[2]}, 
        velocity: Coordinate3D {x:velocity[0],y:velocity[1],z:velocity[2]} 
      })
    }
  }
  hails
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    let parsed = parse(&get_input());
    let result = count_intersections(&parsed,(7.0,27.0));
    assert_eq!(2, result);
  }

  #[test]
  fn test_part2() {
    assert_eq!(47, part2(&get_input()));
  }
}