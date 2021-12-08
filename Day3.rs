
use std::fs;
use std::str;
use std::str::Chars;

#[derive(Copy, Clone)]
struct column {
    zeros: i32,
    ones: i32,
}

struct processedInput {
    columns: [column; 12],
    lines: Vec<i32>,
}

fn processInputLine(line: &str) -> i32 {
    let mut total: i32 = 0;
    
    for c in line.chars() {
        if c == '0' {
            total <<= 1;
            }
                   if c == '1' {
                       total <<= 1;
                               total |= 1;
                       }
                       }
                       
                       return total;
                   }

                   fn processInput(fileName : String) -> processedInput {
                       let mut input: Vec<i32> = Vec::new();
                       
                       let contents = fs::read_to_string(fileName).expect("Could not read file.");
                       
                       for el in contents.lines() {
                           let line_val: i32 = processInputLine(&el);
	                   input.push(line_val);
                       }

                       let mut cols: [column; 12] = [column {zeros:0, ones:0}; 12];

                       for c in &input {
                           for i in 0..12 {
                               if (c >> i)&1 == 1 {
	                           cols[11-i].ones += 1; 
	                       }
	                       else {
	                           cols[11-i].zeros += 1;
	                       }
                           }
                       }                       

                       let pi: processedInput = processedInput {columns: cols, lines: input};
                       
                       return pi;
                   }
                   
                   fn part1(input: &processedInput) {
                       println!("Part 1: ");
                       
                       let mut colsCounted: i32 = 0;
                       
                       let mut gamma: u32 = 0;
                       let mut epsilon: u32 = 0;
                       
                       for i in 0..12 {
                           gamma <<= 1;
                           epsilon <<= 1;
                           if input.columns[i].ones > input.columns[i].zeros {
	                       gamma |= 1;
                           }
                           else if input.columns[i].ones < input.columns[i].zeros {
	                       epsilon |= 1;
                           }
                       }
                       
                       println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
                       println!("Answer: {}", (gamma as u64)*(epsilon as u64));
                   }
                   
                   fn mostCommonBit(col: i32, rows: &Vec<i32>) -> i32 {
                       let mut ones: i32 = 0;
                       let mut zeros: i32 = 0;
                       
                       for r in rows {
                           if (r >> col) & 1 == 1 {
                               ones += 1;
                           }
                           else if (r >> col) & 1 == 0 {
                               zeros += 1;
                           }
                       }

                       if ones > zeros {
                           return 1;
                       }
                       else if zeros > ones {
                           return 0;
                       }
                       else
                       {
                           return 2;
                       }
                   }

                   fn narrow(col: i32, rows: &Vec<i32>, common: bool) -> i32 {
                       let mostCommon: i32 = mostCommonBit(col, rows);

                       let mut nv : Vec<i32> = Vec::new();
                       let mut bitToSelect:i32 = 0;

                       if common {
                           if mostCommon == 1 {
                               bitToSelect = 1;
                           }
                           else if mostCommon == 0 {
                               bitToSelect = 0;
                           }
                           else if mostCommon == 2 {
                               bitToSelect = 1;
                           }
                       }
                       else {
                           if mostCommon == 0 {
                               bitToSelect = 1;
                           }
                           else if mostCommon == 1 {
                               bitToSelect == 0;
                           }
                           else if mostCommon == 2 {
                               bitToSelect == 0;
                           }
                           
                       }
                       
                       for r in rows {
                           if r >> col & 1 == bitToSelect {
                               nv.push(*r);
                           }
                       }

                       if nv.len() == 1 {
                           return nv[0]
                       }
                       else {
                           return narrow(col-1, &nv, common);
                       }
                   }
                   
                 
fn part2(input: &processedInput) {
    println!("Part 2: ");
    let mut oxygen: i32  = 0;
    let mut co: i32 = 0;
    
    oxygen = narrow(11, &input.lines, true);
    co = narrow(11, &input.lines, false);

    println!("Oxygen: {}, CO2: {}, result: {}", oxygen, co, oxygen*co);
}

fn main() {
   let input = processInput("data/Day3.txt".to_string());

   part1(&input);
   part2(&input);
}
