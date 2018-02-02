extern crate regex;

use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Program<'a> {
    pub name: &'a str,
    pub weight: usize,
    pub total_weight: usize,
    pub children: Vec<&'a str>,
    pub parent: Option<&'a str>,
}

fn assemble_program_tree(input: &str) -> HashMap<&str, Program> {
    let program_regex = Regex::new(
        r"(?m)^(?P<name>[[:alpha:]]+) +\((?P<weight>\d+)\)(?: +\->(?P<children>.*))?$",
    ).unwrap();
    
    let children_regex = Regex::new(r"(?P<name>[[:alpha:]]+)").unwrap();

    let mut tree = HashMap::<&str, Program>::new();

    for program_cap in program_regex.captures_iter(input) {
        let program_name = program_cap.name("name").unwrap().as_str();

        let program_weight: usize = program_cap
            .name("weight")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        tree.insert(
            program_name,
            Program {
                name: program_name,
                weight: program_weight,
                total_weight: 0,
                children: Vec::new(),
                parent: None,
            },
        );

        let program = tree.get_mut(program_name).unwrap();
        let children = program_cap.name("children");

        if children.is_none() {
            continue;
        }

        for child_cap in children_regex.captures_iter(children.unwrap().as_str()) {
            let child_name = child_cap.name("name").unwrap().as_str();

            program.children.push(child_name);
        }
    }

    tree
}

fn trace_parents<'a>(tree: &mut HashMap<&'a str, Program<'a>>) -> &'a str {
    let mut parent_list = Vec::new();

    for program in tree.values() {
        for child_name in program.children.iter().cloned() {
            parent_list.push((child_name, program.name));
        }
    }

    for (child_name, parent_name) in parent_list {
        tree.get_mut(child_name).unwrap().parent = Some(parent_name);
    }

    let mut bottom = tree.values().next().unwrap();

    while let Some(parent_name) = bottom.parent {
        bottom = tree.get(parent_name).unwrap();
    }

    bottom.name
}

fn count_total_weight<'a>(tree: &mut HashMap<&'a str, Program<'a>>) {
    let mut total_weights = HashMap::<&str, usize>::new();

    for program in tree.values() {
        if !total_weights.contains_key(program.name) {
            count_program_total_weight(tree, &mut total_weights, program);
        }
    }

    for (program_name, program_weight) in total_weights {
        tree.get_mut(program_name).unwrap().total_weight = program_weight;
    }
}

fn count_program_total_weight<'a>(
    tree: &HashMap<&'a str, Program<'a>>,
    total_weights: &mut HashMap<&'a str, usize>,
    program: &Program<'a>,
) -> usize {
    let mut total_weight = program.weight;

    for child_name in program.children.iter().cloned() {
        if let Some(child_weight) = total_weights.get(child_name) {
            total_weight += child_weight;
            continue;
        }

        total_weight +=
            count_program_total_weight(tree, total_weights, tree.get(child_name).unwrap());
    }

    total_weights.insert(program.name, total_weight);
    total_weight
}

fn find_correct_weight(tree: &HashMap<&str, Program>, program_name: &str) -> Option<usize> {
    let parent = tree.get(program_name).unwrap();

    if parent.children.len() < 3 {
        return None;
    }

    let children_weights: Vec<usize> = parent
        .children
        .iter()
        .map(|c| tree.get(c).unwrap().total_weight)
        .collect();

    let normal_weight = if children_weights[0] == children_weights[1]
        || children_weights[0] == children_weights[2]
    {
        children_weights[0]
    } else if children_weights[1] == children_weights[2] {
        children_weights[1]
    } else {
        return None;
    };

    for (i, &weight) in children_weights.iter().enumerate() {
        if weight == normal_weight {
            continue;
        }

        let child_search_result = find_correct_weight(tree, parent.children[i]);

        if child_search_result.is_some() {
            return child_search_result;
        } else {
            let child = tree.get(parent.children[i]).unwrap();

            return Some(child.weight + normal_weight - child.total_weight);
        }
    }

    None
}

fn search_programs(input: &str) -> (&str, Option<usize>) {
    let mut tree = assemble_program_tree(input);
    let bottom_program = trace_parents(&mut tree);

    count_total_weight(&mut tree);

    let corrected_weight = find_correct_weight(&tree, bottom_program);

    (bottom_program, corrected_weight)
}

fn main() {
    let arg = env::args().skip(1).next();

    if let None = arg {
        println!("Usage: day07 <input file>");
        return;
    }

    let file = File::open(arg.unwrap());

    if let Err(error) = file {
        println!("{}", error.to_string());
        return;
    }

    let mut input = String::new();

    if let Err(error) = file.unwrap().read_to_string(&mut input) {
        println!("{}", error.to_string());
        return;
    }

    let (bottom_program, corrected_weight) = search_programs(input.as_str());

    println!("Bottom program: {}", bottom_program);

    if let Some(weight) = corrected_weight {
        println!("Corrected weight: {}", weight);
    } else {
        println!("Corrected weight: (none)");
    }
}

#[test]
fn test() {
    assert_eq!(
        search_programs(
            "pbga (66)\n\
             xhth (57)\n\
             ebii (61)\n\
             havc (66)\n\
             ktlj (57)\n\
             fwft (72) -> ktlj, cntj, xhth\n\
             qoyq (66)\n\
             padx (45) -> pbga, havc, qoyq\n\
             tknk (41) -> ugml, padx, fwft\n\
             jptl (61)\n\
             ugml (68) -> gyxo, ebii, jptl\n\
             gyxo (61)\n\
             cntj (57)\n"
        ),
        ("tknk", Some(60))
    );
}
