use std::collections::{HashMap, VecDeque};
use dyn_clone::DynClone;

use helper::numerical;

pub fn part1(input:&[String]) -> usize {
  let (mut modules,_in_map,out_map) = parse(input);
  let mut total_low = 0;
  let mut total_high = 0;
  for _ in 0..1000 {
    let res = send_message(&mut modules,&out_map);
    total_low += res.0;
    total_high += res.1;
  }
  (total_low*total_high) as usize
}

pub fn part2(input:&[String]) -> usize {
  let (modules,in_map,out_map) = parse(input);
  let input_modules_rx = in_map.get(&"rx".to_string()).unwrap();
  let mut input_modules = vec![];
  for module in input_modules_rx {
    input_modules.append(&mut in_map.get(module).unwrap().clone())
  }
  let mut counts = vec![];
  for module in input_modules {
    let mut count = 0;
    let mut used_modules = modules.clone();
    while !send_message_until(&module,&mut used_modules,&out_map) {
      count+=1;
    }
    counts.push(count+1);
  }
  numerical::lcm(counts) as usize
}

pub fn send_message(modules:&mut HashMap<String, Box<dyn Module>>, out_map:&HashMap<String, Vec<String>>) -> (u64,u64) {
  let mut messages = VecDeque::new();
  messages.push_back(("button".to_string(),Signal::Low,"broadcaster".to_string()));
  let mut low_count = 0;
  let mut high_count = 0;
  while let Some(message) = messages.pop_front() {
    let from = message.0;
    let signal = message.1;
    let to = message.2;
    if signal.is_high() {high_count+=1} else {low_count+=1}
    // println!("{} -{:?}-> {}",from.clone(),signal,to.clone());
    // let module = modules.get(&to).unwrap();
    if let Some(module) = modules.get_mut(&to) {
      if let Some(response_signal) = module.receive(signal, from.clone()) {
        for out_module in out_map.get(&to).unwrap() {
          messages.push_back((to.clone(),response_signal,out_module.clone()))
        }
      }
    }
  }
  (low_count,high_count)
}

pub fn send_message_until(module_name:&String, modules:&mut HashMap<String, Box<dyn Module>>, out_map:&HashMap<String, Vec<String>>) -> bool {
  let mut messages = VecDeque::new();
  messages.push_back(("button".to_string(),Signal::Low,"broadcaster".to_string()));
  while let Some(message) = messages.pop_front() {
    let from = message.0;
    let signal = message.1;
    let to = message.2;
    if from==*module_name && signal.is_high() {
      return true;
    }
    // println!("{} -{:?}-> {}",from.clone(),signal,to.clone());
    // let module = modules.get(&to).unwrap();
    if let Some(module) = modules.get_mut(&to) {
      if let Some(response_signal) = module.receive(signal, from.clone()) {
        for out_module in out_map.get(&to).unwrap() {
          messages.push_back((to.clone(),response_signal,out_module.clone()))
        }
      }
    }
  }
  false
}

#[derive(Debug,PartialEq, Eq,Clone, Copy)]
pub enum Signal {Low, High}
impl Signal {
  pub fn is_high(self) -> bool {
    self == Signal::High
  }
  pub fn is_low(self) -> bool {
    self == Signal::Low
  }
}
#[derive(Clone)]
pub struct FlipFlop {
  state: bool,
}
impl Module for FlipFlop {
  fn receive(&mut self,signal:Signal,_from:String) -> Option<Signal> {
    // flip state on low signal and send high if on, low if off
    if signal.is_low() {
      self.state = !self.state;
      match self.state {
        true => return Some(Signal::High),
        false => return Some(Signal::Low),
      }
    }
    // do nothing on high signal
    None
  }
}

#[derive(Clone)]
pub struct Conjunction {
  last_messages:HashMap<String,Signal>
}
impl Conjunction {
  fn initialize(&mut self,connections:Vec<String>) {
    for elem in connections {
      self.last_messages.insert(elem, Signal::Low);
    }
  }
}
impl Module for Conjunction {
  fn receive(&mut self,signal:Signal,from:String) -> Option<Signal> {
    self.last_messages.insert(from, signal);
    if self.last_messages.values().all(|s|s.is_high()) {
      Some(Signal::Low)
    } else {
      Some(Signal::High)
    }
  }
}
#[derive(Clone)]
pub struct Broadcaster;
impl Module for Broadcaster {
  fn receive(&mut self,signal:Signal,_from:String) -> Option<Signal> {
    return Some(signal)
  }
}
pub trait Module:DynClone {
  fn receive(&mut self,signal:Signal,from:String) -> Option<Signal>;
}

dyn_clone::clone_trait_object!(Module);

fn parse(strings:&[String]) -> (HashMap<String, Box<dyn Module>>, HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
  let mut modules:HashMap<String, Box<dyn Module>> = HashMap::new();
  let mut out_connections_map:HashMap<String, Vec<String>> = HashMap::new();
  let mut in_connections_map:HashMap<String, Vec<String>> = HashMap::new();
  let mut conjunctions = vec![];
  for line in strings {
    let mut typename_connections_split = line.split(" ->");
    let typename = typename_connections_split.next().unwrap();
    let out_connections:Vec<String> = typename_connections_split.next().unwrap().split(", ").map(|s|String::from(s.trim())).collect();
    let module_type;
    let name;
    if typename == "broadcaster" {
      module_type = 'b';
      name = typename.to_string();
    } else {
      module_type = typename.chars().next().unwrap();
      name = typename.chars().skip(1).collect();
    }
    // println!("name:{}, type: {}, out_connections:{:?}",name,module_type,out_connections);
    match module_type {
      'b' => {
        modules.insert(name.clone(), Box::new(Broadcaster));
      },
      '%' => {
        modules.insert(name.clone(), Box::new(FlipFlop{state:false}));
      },
      '&' => {
        conjunctions.push(name.clone());
      }
      _ => panic!("Unexpcted module type")
    }
    out_connections_map.insert(name.clone(), out_connections.clone());
    for connection in out_connections {
      in_connections_map.entry(connection).and_modify(|x|x.push(name.clone())).or_insert(vec![name.clone()]);
    }
  }
  for conjunction_name in conjunctions {
    let mut conj = Conjunction{last_messages:HashMap::new()};
    conj.initialize(in_connections_map.get(&conjunction_name).unwrap().clone());
    modules.insert(conjunction_name, Box::new(conj));
  }
  (modules,in_connections_map,out_connections_map)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input_1() -> Vec<String> {
    "broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a".lines().map(|s| String::from(s.trim())).collect()
  }
  fn get_input_2() -> Vec<String> {
    "broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1_1() {
    assert_eq!(32000000, part1(&get_input_1()));
  }
  #[test]
  fn test_part1_2() {
    assert_eq!(11687500, part1(&get_input_2()));
  }

  #[test]
  fn low_signal() {
    assert_eq!(true, Signal::High.is_high());
    assert_eq!(false, Signal::High.is_low());
  }
  #[test]
  fn high_signal() {
    assert_eq!(false, Signal::Low.is_high());
    assert_eq!(true, Signal::Low.is_low());
  }
}