use std::collections::{HashMap, VecDeque};

pub fn part1(input:&[String]) -> usize {
  let (workflow_map,parts) = parse(input);
  let mut total = 0;
  for part in parts {
    let mut curr_workflow = "in".to_string();
    loop {
      let workflow = workflow_map.get(&curr_workflow).unwrap();
      for rule in workflow {
        if let Some(curr_group_res) = rule.apply(&part) {
          curr_workflow = curr_group_res;
          break;
        }
      }
      // println!("->{}",curr_workflow);
      if curr_workflow=="A" {
        total+=part.values.iter().sum::<u64>();
        break;
      } else if curr_workflow=="R" {
        break;
      }
    }
  }
  total as usize
}
pub fn part2(input:&[String]) -> usize {
  let (workflow_map,_) = parse(input);
  let total = count_combinations(&workflow_map);
  total as usize
}

fn count_combinations(workflows_map: &HashMap<String, Vec<Rule>>) -> u64 {
  let mut ranges = vec![];
  for _ in 0..4 {
    ranges.push((1,4000));
  }
  let mut queue = VecDeque::new();
  queue.push_back(("in",ranges));
  let mut accepted_ranges = vec![];
  while let Some((state,mut ranges)) = queue.pop_front() {
    // println!("{:?}",(state,&ranges));
    if state == "A" {
      accepted_ranges.push(ranges);
      continue;
    }
    if state == "R" {
      continue;
    }
    let workflow = workflows_map.get(&state.to_string()).unwrap();
    for rule in workflow {
      let mut new_ranges = ranges.clone();
      if rule.rule_type==RuleType::Default {
        queue.push_back((rule.group.as_str(),ranges));
        break;
      }
      match rule.rule_type {
        RuleType::Bigger => {
          if rule.value>new_ranges[rule.variable].0 {new_ranges[rule.variable].0=rule.value+1}
          if rule.value<ranges[rule.variable].1 {ranges[rule.variable].1=rule.value}
          queue.push_back((rule.group.as_str(),new_ranges.clone()));
        },
        RuleType::Smaller => { //x<10 (1,4000)
          if rule.value<new_ranges[rule.variable].1 {new_ranges[rule.variable].1=rule.value-1}
          if rule.value>ranges[rule.variable].0 {ranges[rule.variable].0=rule.value}
          queue.push_back((rule.group.as_str(),new_ranges.clone()));
        },
        _ => panic!("")
      }
    }
  }
  let mut total = 0;
  for v in accepted_ranges.iter().map(|v|v.iter().map(|r|combinations(*r)).fold(1, |acc,v|acc*v)) {
    total+=v;
  }
  total

}

fn combinations(range:(u64,u64)) -> u64 {
  if range.1<range.0 {
    return 0
  } else {
    return range.1+1-range.0
  }
}

fn parse(strings:&[String]) -> (HashMap<String,Vec<Rule>>,Vec<Part>) {
  // parse workflows until newline
  let mut workflows = HashMap::new();
  for s in strings {
    if s.is_empty() {break}
    // parse workflow name
    let mut name_rules_split = s.split("{");
    let workflow_name = name_rules_split.next().unwrap().to_string();
    // parse rules
    let rules_split = name_rules_split.next().unwrap().split(",");
    let mut rules = vec![];
    for rule in rules_split {
      let operator ;
      let rule_type;
      if rule.contains('>') {
        rule_type = RuleType::Bigger;
        operator = ">"
      } else if rule.contains('<') {
        rule_type = RuleType::Smaller;
        operator = "<"
      } else {
        // this is the default rule, take only the group
        let default_group:String = rule.chars().filter(|c|c.is_alphabetic()).collect();
        rules.push(Rule{
          rule_type:RuleType::Default,
          variable:0,
          value:0,
          group:default_group
        });
        break;
      }
      let mut rule_group_split = rule.split(":");
      let mut rule_value_split = rule_group_split.next().unwrap().split(operator);
      let variable = rule_value_split.next().unwrap();
      let variable_index = match variable {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!()
      };
      let value:u64 = rule_value_split.next().unwrap().parse().unwrap();
      let group = rule_group_split.next().unwrap().to_string();
      rules.push(Rule { rule_type , variable:variable_index, value, group});
    }
    workflows.insert(workflow_name, rules);
  }
  // parse parts
  let string_parts = strings.iter().filter(|s|s.starts_with("{"));
  let mut parts = vec![];
  for s in string_parts {
    let assignments_split = s.split(",");
    let mut part_array = [0;4];
    for assignment in assignments_split {
      let (variable,value) = assignment.split_once("=").unwrap();
      let clean_variable:String = variable.chars().filter(|x|x.is_alphabetic()).collect();
      let clean_value = value.chars().filter(|x|x.is_numeric()).collect::<String>().parse().unwrap();
      match clean_variable.as_str() {
        "x" => part_array[0] = clean_value,
        "m" => part_array[1] = clean_value,
        "a" => part_array[2] = clean_value,
        "s" => part_array[3] = clean_value,
        _ => panic!("Unexpected variable in assignment")
      }
    }
    parts.push(Part{values:part_array})

  }
  (workflows,parts)
}
#[derive(Debug)]
pub struct Part {
  values:[u64;4]
}

#[derive(Debug)]
pub struct Rule {
  rule_type:RuleType,
  variable:usize,
  value:u64,
  group:String
}
impl Rule {
  pub fn apply(&self, part:&Part) -> Option<String> {
    if self.rule_type==RuleType::Default {
      return Some(self.group.clone());
    }
    match self.rule_type {
        RuleType::Bigger => if part.values[self.variable]>self.value {return Some(self.group.clone())},
        RuleType::Smaller => if part.values[self.variable]<self.value {return Some(self.group.clone())},
        RuleType::Default => panic!(),
    }
    None
  }
}
#[derive(Debug,PartialEq, Eq,Clone,Copy)]
enum RuleType {
  Bigger,Smaller,Default
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}
    
    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(19114, part1(&get_input()));
  }

  #[test]
  fn test_part2() {
    assert_eq!(167409079868000, part2(&get_input()));
  }
}