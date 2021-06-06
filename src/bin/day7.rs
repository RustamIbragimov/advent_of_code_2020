use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
struct InnerBugs {
    count: u32,
    name: String,
}

#[derive(Debug)]
struct Bug {
    name: String,
    bugs: Vec<InnerBugs>,
}

impl Bug {
    fn parse(line: &str) -> Self {
        let mut truncated_line = line.to_string();
        truncated_line.truncate(line.len() - 1);
        let split_line: Vec<_> = truncated_line.split("contain").collect();
        let name = Bug::format_bug_name(split_line[0]);
        let bugs = Bug::parse_inner_bugs(split_line[1].trim());

        Self { name, bugs }
    }

    fn parse_inner_bugs(line: &str) -> Vec<InnerBugs> {
        let mut bugs: Vec<InnerBugs> = Vec::new();
        if line != "no other bags" {
            let inner_bugs: Vec<_> = line.split(",").collect();
            for inner_bug in inner_bugs {
                let trimmed_inner_bug = inner_bug.trim();
                let mut last_digit_index = 0;
                for c in trimmed_inner_bug.chars() {
                    if !c.is_numeric() {
                        break;
                    }
                    last_digit_index += 1;
                }
                let count = trimmed_inner_bug[..last_digit_index]
                    .parse::<u32>()
                    .unwrap();
                let inner_bug_name =
                    Bug::format_bug_name(&trimmed_inner_bug[last_digit_index + 1..]);
                bugs.push(InnerBugs {
                    count,
                    name: inner_bug_name,
                });
            }
        }
        bugs
    }

    fn format_bug_name(bug_name: &str) -> String {
        bug_name
            .replace(" bags", "")
            .replace(" bag", "")
            .trim()
            .to_string()
    }
}

#[derive(Debug)]
struct ReversedGraph {
    g: HashMap<String, HashSet<String>>,
}

impl ReversedGraph {
    fn build(bugs: &Vec<Bug>) -> Self {
        let mut graph = HashMap::<String, HashSet<String>>::new();
        for bug in bugs {
            for inner_bug in &bug.bugs {
                let bug_names = graph
                    .entry(inner_bug.name.clone())
                    .or_insert(HashSet::new());
                bug_names.insert(bug.name.clone());
            }
        }
        Self { g: graph }
    }

    fn count(&self, seen: &mut HashSet<String>, current_bag_name: String) {
        match self.g.get(&current_bag_name) {
            None => {}
            Some(neighbors) => {
                for neighbor in neighbors {
                    seen.insert(neighbor.clone());
                    self.count(seen, neighbor.clone());
                }
            }
        }
    }
}

#[derive(Debug)]
struct Graph<'a> {
    g: HashMap<String, HashSet<&'a InnerBugs>>,
}

impl<'a> Graph<'a> {
    fn build_graph(bugs: &'a Vec<Bug>) -> Self {
        let mut graph = HashMap::<String, HashSet<&InnerBugs>>::new();
        for bug in bugs {
            let bug_names = graph.entry(bug.name.clone()).or_insert(HashSet::new());
            for inner_bug in &bug.bugs {
                bug_names.insert(inner_bug);
            }
        }
        Self { g: graph }
    }

    fn count(&self, current_bag_name: String) -> u32 {
        match self.g.get(&current_bag_name) {
            None => 0,
            Some(neighbors) => {
                let mut total_count = 0;
                for neighbor in neighbors {
                    let inner_count = self.count(neighbor.name.clone());
                    total_count += inner_count * neighbor.count + neighbor.count;
                }
                total_count
            }
        }
    }
}

fn main() {
    let content = fs::read_to_string("inputs/day7.txt").expect("Expected a file");
    let mut bugs: Vec<Bug> = Vec::new();
    for line in content.lines() {
        bugs.push(Bug::parse(line));
    }

    let inverse_graph = ReversedGraph::build(&bugs);
    let mut seen = HashSet::<String>::new();
    inverse_graph.count(&mut seen, "shiny gold".to_string());
    println!("Part 1: {}", seen.len());

    let graph = Graph::build_graph(&bugs);
    let res = graph.count("shiny gold".to_string());
    println!("Part 2: {}", res);
}
