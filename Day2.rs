
use std::fs;
use std::str;
use std::str::Chars;

enum direction {
     FORWARD,
     UP,
     DOWN,
}

struct l {
       dir: direction,
       dist: i32,
}

fn startsWith(haystack: &mut Chars, needle: &Chars) -> bool {
   let mut hCur: Chars<'_> = (*haystack).clone();
   let mut nCur: Chars<'_> = (*needle).clone();

   loop {
      let hChar: Option<char> = hCur.next();
      let nChar: Option<char> = nCur.next();

      if nChar == None {
        *haystack = hCur;
	return true;
	}
      else if hChar == None {
        return false;
        }
      else {
        if nChar.unwrap() != hChar.unwrap() {
	  return false;
	  }	
	}
   }
}

fn processInputLine(line: &str) -> l {
   let mut it: Chars = (*line).chars();

   let mut line_rep : l = l {dir: direction::FORWARD, dist: 0,};

   if startsWith(&mut it, &"forward".chars()) {
      line_rep.dir = direction::FORWARD;
   }
   else if startsWith(&mut it, &"down".chars()) {
      line_rep.dir = direction::DOWN;
   }
   else if startsWith(&mut it, &"up".chars()) {
      line_rep.dir = direction::UP;
   }

   let mut s: &str = it.as_str();
   let num_parse = s.parse::<i32>();
   if num_parse.is_ok() {
      let no: i32 = num_parse.unwrap();
      line_rep.dist = no;
   }

   return line_rep;
}

fn processInput(fileName : String) -> Vec<l> {
   let mut input : Vec<l> = Vec::new();

   let contents = fs::read_to_string(fileName).expect("Could not read file.");

   for el in contents.lines() {
        let line_rep: l = processInputLine(&el);
	input.push(line_rep);
   }

   return input;
}

fn part1(input: &Vec<l>) {
   println!("Part 1: ");

   let mut x_pos: i32 = 0;
   let mut y_pos: i32 = 0;

   for i in input {
       match i.dir {
       	     direction::FORWARD => x_pos += i.dist,
	     direction::UP => y_pos -= i.dist,
	     direction::DOWN => y_pos += i.dist,
       }
   }

   let result: i32 = x_pos * y_pos;
   println!("Answer: {}", result);
}

fn part2(input: &Vec<l>) {
   println!("Part 2: ");

   let mut x_pos: i32 = 0;
   let mut y_pos: i32 = 0;
   let mut aim: i32 = 0;

   for i in input {
       match i.dir {
       	     direction::FORWARD => {
	         x_pos += i.dist;
	         y_pos += aim*i.dist;
		 },
	     direction::UP => aim -= i.dist,
	     direction::DOWN => aim += i.dist,
       }
   }

   let result: i32 = x_pos * y_pos;
   println!("Answer: {}", result);
}

fn main() {
   let input = processInput("data/Day2.txt".to_string());

   part1(&input);
   part2(&input);
}