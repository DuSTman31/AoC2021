
use std::fs;
use std::str;
use std::str::Chars;


fn process_input(fileName : String) -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    
    let contents = fs::read_to_string(fileName).expect("Could not read file.");
    
    for el in contents.lines() {
        input.push(el.to_string());
    }

    return input;
}

fn is_open(c: char) -> bool {
    if c == '{' || c == '[' || c == '<' || c == '('{
        return true;
    }
    else {
        return false;
    }
}

fn complement(c: char) -> char {
    match c{
        '(' => return ')',
        '{' => return '}',
        '[' => return ']',
        '<' => return '>',
        ')' => return '(',
        '}' => return '{',
        ']' => return '[',
        '>' => return '<',
        default => return ' ',
    }
}

fn error_value(c: char) -> i32 {
    match c {
        ')' => return 3,
        ']' => return 57,
        '}' => return 1197,
        '>' => return 25137,
        default => return 0,
    }
}

fn scan_for_syntax_error(line: &String) -> Option<i32> {
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        if is_open(c) {
            stack.push(c)
        }
        else {
            let mc  = complement(c);
            if mc == ' ' {
                return None;
            }

            if stack.pop().unwrap() != mc {
                return Some(error_value(c));
            }
        }
    }
    return None;
}
    
fn part1(input: &Vec<String>) {
    println!("Part 1: ");
    
    let vals = input.iter().map(|x| {scan_for_syntax_error(x)});
    let mut total: i32 = 0;

    for i in vals{
        if i.is_some() {
            total += i.unwrap();
        }
    }
    println!("{}", total);
}

fn close_score(c: char) -> i64 {
    return match c{
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        default => 0,
    }
}

fn close_stack(s: &mut Vec<char>) -> i64 {
    let mut total: i64 = 0;

    while s.len() != 0 {
        let c: char = s.pop().unwrap();
        let cc: char = complement(c);
        total *= 5;
        total += close_score(cc);
    }

    return total;
}

fn part2(input: &Vec<String>) {
    println!("Part 2: ");

    let vals = input.iter().filter(|x| {scan_for_syntax_error(x).is_none()});
    let mut scores: Vec<i64> = Vec::new();

    for line in vals {
        let mut stack: Vec<char> = Vec::new();

        for c in line.chars() {
            if is_open(c) {
                stack.push(c)
            }
            else {
                stack.pop().unwrap();
            }
        }
        
        scores.push(close_stack(&mut stack));
    }

    scores.sort();

    for s in &scores {
        println!("{}", s);
    }
    
    let result: i64 = scores[(scores.len()/2)];
    
    println!("Result: {}", result);
}

fn main() {
   let input = process_input("data/Day10.txt".to_string());

   part1(&input);
   part2(&input);
}
