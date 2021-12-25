
use std::fs;
use std::str;
use std::str::Chars;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::cmp::Ordering;



struct processed_input {
    pattern: Vec<bool>,
    starting_picture: Vec<Vec<bool>>,
}

fn processInput(fileName : String) -> processed_input {
    let contents = fs::read_to_string(fileName).expect("Could not read file.");
    
    let mut l = contents.lines();

    let pattern_str = l.next().unwrap();
    let mut pattern: Vec<bool> = Vec::new();
    for c in pattern_str.chars() {
        if c == '.' {
            pattern.push(false);
        }
        else if c == '#' {
            pattern.push(true);
        }

    }
    l.next();

    let mut sb: Vec<Vec<bool>> = Vec::new();

    for line in l {
        let mut bl: Vec<bool> = Vec::new();
        for c in line.chars() {
            if c == '.' {
                bl.push(false);
            }
            else if c == '#' {
                bl.push(true);
            }
        }
        sb.push(bl);
    }

    return processed_input {pattern: pattern, starting_picture: sb};
}


fn get_pixel(input_image: &Vec<Vec<bool>>, x: i32, y: i32, def: bool) -> bool {
    if x < 0 || x >= input_image[0].len() as i32 {
        return def;
    }
    if y < 0 || y >= input_image.len() as i32 {
        return def;
    }
    return input_image[y as usize][x as usize];
}

fn get_surrounding_pixel_block(input_image: &Vec<Vec<bool>>, x: i32, y: i32, def: bool) -> i32 {
    let x_start = x - 1;
    let y_start = y - 1;

    let mut i: i32 = 0;
    for cy in y_start..y+2 {
        for cx in x_start..x+2 {
            i <<= 1;
            if get_pixel(&input_image, cx, cy, def) {
                i |= 1;
            }
        }
    }

    return i;
}

fn process_image(rules: &Vec<bool>, input_image: &Vec<Vec<bool>>, flipped: bool) -> Vec<Vec<bool>> {
 
    let old_x_size: i32 = input_image[0].len() as i32;
    let old_y_size: i32 = input_image.len() as i32;
    let new_x_size: i32 = old_x_size+2;
    let new_y_size: i32 = old_y_size+2;

    let mut out_image: Vec<Vec<bool>> = Vec::with_capacity(new_y_size as usize);
    
    for y in 0..new_y_size {
        let mut out_line: Vec<bool> = Vec::with_capacity(new_x_size as usize);
        for x in 0..new_x_size {
             let i  = rules[get_surrounding_pixel_block(&input_image, x-1, y-1, flipped) as usize];
            if i==true {
                out_line.push(true);
            }
            else {
                out_line.push(false);
            }

        }
        out_image.push(out_line);
    }
  
    return out_image;
}


fn repeat_enhance(rules: &Vec<bool>, input_image: &Vec<Vec<bool>>, times: i32) -> Vec<Vec<bool>> {
    let mut border: bool = false;

    let mut ip = input_image.clone();
    for i in 0..times {
        let op = process_image(&rules, &ip, border);
        if !border && rules[0] {
            border = true;
        }
        else if border && !rules[511] {
            border = false;
        }
        ip = op;
    }
    return ip;
}

fn part1(input: &processed_input) {
    println!("Part 1: ");

    let exp_image: Vec<Vec<bool>> = repeat_enhance(&input.pattern, &input.starting_picture, 2);

    let mut lit_pixels: i32 = 0;
    for i in exp_image.iter() {
        for j in i.iter() {
            if *j {
                lit_pixels += 1;
            }
        } 
    }

    println!("{} pixels are lit.", lit_pixels)
}

fn part2(input: &processed_input) {
    println!("Part 2: ");

    let mut exp_image: Vec<Vec<bool>> = repeat_enhance(&input.pattern, &input.starting_picture, 50);

    
    let mut lit_pixels: i32 = 0;
    for i in exp_image.iter() {
        for j in i.iter() {
            if *j {
                lit_pixels += 1;
            }
        } 
    }

    println!("{} pixels are lit.", lit_pixels)
}


fn main() {
    let input = processInput("data/Day20.txt".to_string());

    part1(&input);
    part2(&input);
    
}
