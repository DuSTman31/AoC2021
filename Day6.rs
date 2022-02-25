
use std::fs;
use std::str;
use std::str::Chars;
use std::collections::BTreeMap;


struct processedInput {
    data: Vec<i64>,
}

fn processInput(fileName : String) -> processedInput {
    let mut input: Vec<i64> = Vec::new();
    
    let contents = fs::read_to_string(fileName).expect("Could not read file.");
    
    let mut digits: Vec<i64> = Vec::new();
    
    for i in contents.split(",") {
        let ii = i.trim_end().parse::<i64>().unwrap();
        digits.push(ii);
    }
        
    return processedInput {data: digits};
}

fn do_step(input: &Vec<i64>) -> Vec<i64> {
    let mut res: Vec<i64> = Vec::new();

    let mut no_zeroes: i64 = 0;
    for i in input.iter() {
        if *i==0  {
            no_zeroes += 1;
        }
    }

    for i in input.iter() {
        let mut dig: i64 = *i;
        if dig==0 {
            dig=6;
        }
        else {
            dig -= 1;
        }
        res.push(dig);
    }

    for i in 0..no_zeroes {
        res.push(8);
    }
    
    return res;
}

fn part1(input: &processedInput) {
    println!("Part 1: ");

    let mut data = input.data.clone();

    for i in 0..80 {
        data = do_step(&data);
    }

    println!("{} lanternfish", data.len());
}
                   
                 
fn part2(input: &processedInput) {
    println!("Part 2: ");

    let mut numbers: Vec<i64> = Vec::new();
    for i in 0..9 {
        numbers.push(0);
    }

    for i in input.data.iter() {
        numbers[*i as usize] += 1;
    }

    for i in 0..256 {
        let mut new_numbers: Vec<i64> = Vec::new();

        for i in 0..9 {
            new_numbers.push(0);
        }

        new_numbers[0] = numbers[1];
        new_numbers[1] = numbers[2];
        new_numbers[2] = numbers[3];
        new_numbers[3] = numbers[4];
        new_numbers[4] = numbers[5];
        new_numbers[5] = numbers[6];
        new_numbers[6] = numbers[7] + numbers[0];
        new_numbers[7] = numbers[8];
        new_numbers[8] = numbers[0];

        numbers = new_numbers;
    }

    let mut total: i64 = 0;

    for i in numbers.iter() {
        total += i;
    }

    println!("{}, lanternfish", total);
}

fn main() {
    let input = processInput("data/Day6.txt".to_string());
    
    part1(&input);
    part2(&input);
}
