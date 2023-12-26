use std::vec;

pub fn part1(input:&[String]) -> usize {
  let (seeds,maps_list) = parse(input);
  let locations:Vec<usize> = seeds.iter().map(|seed|seed_to_location(*seed, &maps_list)).collect();
  *locations.iter().min().unwrap()
}
pub fn part2(input:&[String]) -> usize {
  // BRUTE FORCE
  let (seeds,maps_list) = parse(input);
  let mut i = 0;
  let mut minimum = usize::MAX;
  while i<seeds.len()-1 {
    for j in seeds[i]..seeds[i]+seeds[i+1] {
      let location = seed_to_location(j, &maps_list);
      if location<minimum {
        minimum = location;
      }
    }
    i+=2;
  }
  minimum
}

fn seed_to_location(seed:usize,maps_list:&Vec<Vec<ResourceMap>>) -> usize {
  let mut source = seed;
  for map in maps_list {
    let mut conversion_opt = None;
    for little_map in map {
      if little_map.is_in_range(source) {
        conversion_opt = little_map.convert(source);
      }
    }
    source = conversion_opt.unwrap_or(source);
  }
  source
}

#[derive(Debug)]
struct ResourceMap {
  dest_range_start: usize,
  source_range_start: usize,
  range_len: usize
}
impl ResourceMap {
    pub fn convert(&self, source:usize) -> Option<usize> {
      if self.is_in_range(source) {
        return Some(self.dest_range_start + (source - self.source_range_start))
      }
      None
    }
    pub fn is_in_range(&self,source:usize) -> bool {
      source >= self.source_range_start && source < self.source_range_start+self.range_len
    }
}

fn parse(strings:&[String]) -> (Vec<usize>,Vec<Vec<ResourceMap>>) {
  // parse first line that is seeds
  let seeds_string = strings[0].split("seeds: ").last().unwrap();
  let seeds:Vec<usize> = seeds_string.split(" ").map(|s|s.parse::<usize>().unwrap()).collect();

  // start parsing from 4th line until an empty line -> map
  let mut i = 3;
  let mut list_maps:Vec<Vec<ResourceMap>> = vec![];
  while i<strings.len() {
    let mut maps = vec![];
    while i<strings.len() && !strings[i].is_empty()  {
      let mut map = strings[i].split(" ").map(|s|s.parse::<usize>().unwrap());
      maps.push(ResourceMap {
        dest_range_start: map.next().unwrap(),
        source_range_start: map.next().unwrap(),
        range_len: map.next().unwrap()
      });
      i+=1;
    }
    
    list_maps.push(maps);
    i+=2
  }
  (seeds,list_maps)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(35, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(46, part2(&get_input()));
  }
}