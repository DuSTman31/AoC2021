
use std::fs;
use std::str;

fn triSum(a: [i32; 3]) -> i32 {
   let tot : i32 = a[0] + a[1] + a[2];
   return tot;
}

fn triAdv(a: [i32; 3], b: &i32) -> [i32; 3] {
   let mut r: [i32; 3] = a;
   r[0] = r[1];
   r[1] = r[2];
   r[2] = *b;
   return r;
}

fn processInput(fileName : String) -> Vec<i32> {
   let mut input : Vec<i32> = Vec::new();

   let contents = fs::read_to_string(fileName).expect("Could not read file.");

   for el in contents.lines() {
       let i: i32 = el.parse().unwrap_or(0);
       
       input.push(i)
   }

   return input;
}

fn part1(input: &Vec<i32>) {
   println!("Part 1: ");
   let mut larger : i32 = 0;
   let mut begun : bool = false;
   let mut previous : i32 = 0;
   for i in input {

       if begun {
       	  if i>&previous {
	  	  larger += 1;
	  }
       }
       else {
       	    begun = true;
       }

       previous = *i;
   }
   
   println!("Larger: {}", larger);
}

fn part2(input: &Vec<i32>) {
   println!("Part 2: ");
   let mut prev : [i32; 3] = [0, 0, 0];
   let mut first : i32 = 0;
   let mut larger : i32 = 0;
   for i in input {
       if first > 2 {
       	  let p1 : i32 = triSum(prev);
       	  prev = triAdv(prev, i);
	  let p2 : i32 = triSum(prev);
	  if p2 > p1 {
	     larger += 1;
	  }
       }
       else
       {
	prev = triAdv(prev, i);
	first += 1;
       }
   }
   println!("Larger: {}", larger);
}

fn main() {
   println!("Hello, world!");

   let input = processInput("data/Day1.txt".to_string());

   part1(&input);

   part2(&input);
}