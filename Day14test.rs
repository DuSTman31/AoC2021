
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

struct expansion_iter<'l, T: Iterator> {
    base_iter: T,
    status: (T::Item, T::Item),
    started: bool,
    rules: & 'l BTreeMap<(T::Item, T::Item), T::Item>,
    exp_char: T::Item,
    has_exp_char: bool,
    ended: bool,
}

impl<'l, T: Iterator> expansion_iter<'l, T> where <T as Iterator>::Item : Ord + Copy{
    fn new(bi: T, rules: &'l BTreeMap<(T::Item, T::Item), T::Item>, c: T::Item) ->  expansion_iter<'l, T> {
        let n: expansion_iter<'l, T> = expansion_iter {base_iter: bi, status: (c, c), started: false, rules: rules, exp_char: c,  has_exp_char: false, ended: false,};
        return n;
    }
}

impl<'l, T: Iterator> Iterator for expansion_iter<'l, T> where <T as Iterator>::Item : Ord + Copy{
    type Item = T::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            match self.base_iter.next() {
                Some(n) => {self.status.0 = self.status.1;
                            self.status.1 = n;
                            self.started = true;},
                None => {return None;},
            }
        }

        if self.has_exp_char {
            self.has_exp_char = false;
            return Some(self.exp_char);
        }
        else {
            match self.base_iter.next() {
                Some(n) => {self.status.0 = self.status.1;
                            self.status.1 = n;
                            if self.rules.contains_key(&self.status) {
                                self.has_exp_char = true;
                                self.exp_char = *self.rules.get(&self.status).unwrap();
                            }
                            return Some(self.status.0);
                },
                None => {
                    if self.ended {
                        return None;
                    }
                    else {
                        self.ended = true;
                        return Some(self.status.1);
                    }
                },
            }
        }
    }
}

fn part1(input: &processed_input) {
    println!("Part 1: ");

    let mut template =  input.template.clone();
    let mut template_iter = template.chars();
    let it = expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(template_iter, &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c');

    let mut totals: BTreeMap<char, i32> = BTreeMap::new();
    
    for x in it {
        if totals.contains_key(&x) {
            let mut xx = totals.get_mut(&x);
            *xx.unwrap() += 1;
        }
        else {
            totals.insert(x, 1);
        }
    }

    let mut highest: (char, i32) = ('x' ,0);
    let mut highest_set: bool = false;
    let mut lowest: (char, i32) = ('x', 0);
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


fn part2(input: &processed_input) {
    println!("Part 2: ");

    let mut template =  input.template.clone();
    let mut template_iter = template.chars();
    let it = expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(expansion_iter::new(template_iter, &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c'), &input.rules, 'c');

    let mut totals: BTreeMap<char, i64> = BTreeMap::new();
    let mut amt_done: i64 = 0;
    
    for x in it {
        if totals.contains_key(&x) {
            let mut xx = totals.get_mut(&x);
            *xx.unwrap() += 1;
        }
        else {
            totals.insert(x, 1);
        }
        amt_done += 1;
        if amt_done % 1000000000 == 0 {
            println!("{} billion done", amt_done/1000000000);
            return;
        }
    }

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

fn main() {
   let input = process_input("data/Day14.txt".to_string());

    println!("{}", input.template);
    
   part1(&input);
   part2(&input);
}
