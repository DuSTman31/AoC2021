
use std::fs;
use std::str;
use std::str::Chars;
use std::collections::BTreeMap;
use std::cmp::Ordering;

struct processed_input {
    template: String,
    rules: BTreeMap<(char, char), char>, 
}

fn process_input(fileName : String) -> processed_input {
    let mut template: String = String::new();
    let mut rules: BTreeMap<(char, char), char> = BTreeMap::new();
    
    let contents = fs::read_to_string(fileName).expect("Could not read file.");

    let mut line_iter = contents.lines();
    template = line_iter.next().unwrap().to_string();
    line_iter.next();
    
    for l in line_iter {
        let mut c_iter = l.chars();
        
        let a: char = c_iter.next().unwrap();
        let b: char = c_iter.next().unwrap();

        c_iter.next();
        c_iter.next();
        c_iter.next();
        c_iter.next();
        
        let c: char = c_iter.next().unwrap();
        rules.insert((a,b), c);

    }

    let x: processed_input = processed_input {template: template, rules: rules};
    return x;
}

fn exp_pass(rules: &BTreeMap<(char, char), char>, digraph_occurrences: &BTreeMap<(char, char), i64>) -> BTreeMap<(char, char), i64> {
    let mut next_digraph_occurrences: BTreeMap<(char, char), i64> = BTreeMap::new();

    for ((ka, kb), v) in digraph_occurrences.iter() {
        if rules.contains_key(&(*ka, *kb)) {
            let c: char = *rules.get(&(*ka, *kb)).unwrap();
            
            if next_digraph_occurrences.contains_key(&(*ka, c)) {            
                let mut xx = next_digraph_occurrences.get_mut(&(*ka, c));
                *xx.unwrap() += v;
            }
            else {
                next_digraph_occurrences.insert((*ka, c), *v);
            }

            if next_digraph_occurrences.contains_key(&(c, *kb)) {            
                let mut xx = next_digraph_occurrences.get_mut(&(c, *kb));
                *xx.unwrap() += v;
            }
            else {
                next_digraph_occurrences.insert((c, *kb), *v);
            }
            
        }
        else
        {
            next_digraph_occurrences.insert((*ka, *kb), *v);
        }
    }
    
    return next_digraph_occurrences;
}

fn digraph_to_char_occurrences(digraph_occurrences: &BTreeMap<(char, char), i64>) -> BTreeMap<char, i64> {
    let mut co: BTreeMap<char, i64> = BTreeMap::new();

    for ((ka, kb), v) in digraph_occurrences.iter() {
        if co.contains_key(ka) {
            let mut xx = co.get_mut(ka);
            *xx.unwrap() += v;
        }
        else {
            co.insert(*ka, *v);
        }
    }
    
    return co;
}

fn no_passes(input: &processed_input, passes: i32) {
    let mut digraph_occurrence: BTreeMap<(char, char), i64> = BTreeMap::new();;

    let mut template_iter = input.template.chars();
    let mut dg: (char, char) = ('a', template_iter.next().unwrap());

    for c in template_iter {
        dg = (dg.1, c);

        if digraph_occurrence.contains_key(&dg) {
            let mut xx = digraph_occurrence.get_mut(&dg);
            *xx.unwrap() += 1;
        }
        else {
            digraph_occurrence.insert(dg, 1);
        }
    }

    for i in 0..passes {
        digraph_occurrence = exp_pass(&input.rules, &digraph_occurrence);
    }

    let mut totals: BTreeMap<char, i64> = digraph_to_char_occurrences(&digraph_occurrence);
    let xx = totals.get_mut(&input.template.chars().last().unwrap());
    *xx.unwrap() += 1;
    
    let mut highest: (char, i64) = ('x' ,0);
    let mut highest_set: bool = false;
    let mut lowest: (char, i64) = ('x', 0);
    let mut lowest_set: bool = false;

    
    for (k,v) in totals {
        if !highest_set {
            highest = (k, v);
            highest_set = true;
        }
        else if highest.1 < v {
            highest = (k, v);
        }
        
        if !lowest_set {
            lowest = (k, v);
            lowest_set = true;
        }
        else if lowest.1 > v {
            lowest = (k, v);
        }

    }

    println!("Highest: {}, Lowest: {}, answer: {}", highest.1, lowest.1, highest.1 - lowest.1);
}

fn part1(input: &processed_input) {
    println!("Part 1: ");

    no_passes(input, 10);
}


fn part2(input: &processed_input) {
    println!("Part 2: ");


    no_passes(input, 40);
}

fn main() {
   let input = process_input("data/Day14.txt".to_string());

    println!("{}", input.template);
    
   part1(&input);
   part2(&input);
}
