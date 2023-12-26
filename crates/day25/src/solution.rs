use std::collections::HashMap;

use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::Result;

pub fn part1(input:&[String]) -> usize {
  let graph = parse(input);
  let min_cut: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph,|_| Ok(1));
  let cut_size = min_cut.unwrap().unwrap().1.len();
  cut_size*(graph.node_count()-cut_size) as usize
}
pub fn part2(_input:&[String]) -> usize {
  // let parsed = parse(input);
  1
}

fn parse(strings:&[String]) -> UnGraph<&str, i32> {
  let mut g = UnGraph::new_undirected();
  let mut nodes = HashMap::new();
  for line in strings {
    let mut split =line.split(": ");
    let from = split.next().unwrap();
    if !nodes.contains_key(from) {
      let node_index = g.add_node(from);
      nodes.insert(from, node_index);
    }
    let from_index = nodes.get(from).unwrap().clone();
    let to = split.next().unwrap().split(" ");
    for elem in to {
      if !nodes.contains_key(elem) {
        let node_index = g.add_node(elem);
        nodes.insert(elem, node_index);
      }
      let to_index = nodes.get(elem).unwrap();
      g.add_edge(from_index, *to_index, 1);
    }
  }
  g
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    "jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr".lines().map(|s| String::from(s.trim())).collect()
  }

  #[test]
  fn test_part1() {
    assert_eq!(54, part1(&get_input()));
  }
}