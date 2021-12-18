
use std::fs;
use std::str;
use std::str::Chars;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::cmp::Ordering;

#[derive(Copy, Clone)]

struct coord {
    x: i32,
    y: i32,        
}

impl PartialEq for coord {
    fn eq(&self, other: &Self) -> bool  {
        if self.x == other.x {
            return self.y == other.y;
        }
        else {
            return false;
        }
    }
}

impl Eq for coord {}

impl PartialOrd for coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       if self.x < other.x {
            return Some(Ordering::Less);
        }
        else if self.x > other.x {
            return Some(Ordering::Greater);
        }
        else {
            if self.y < other.y {
                return Some(Ordering::Less);
            }
            else if self.y > other.y {
                return Some(Ordering::Greater);
            }
            else {
                return Some(Ordering::Equal);
            }
        }
    }
}

impl Ord for coord {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x < other.x {
            return Ordering::Less;
        }
        else if self.x > other.x {
            return Ordering::Greater;
        }
        else {
            if self.y < other.y {
                return Ordering::Less;
            }
            else if self.y > other.y {
                return Ordering::Greater;
            }
            else {
                return Ordering::Equal;
            }
        }
    }
}


fn add_2d_coord(a: &coord, b: &coord) -> coord {
    let mut c: coord = coord {x:0, y:0,};
    c.x = a.x + b.x;
    c.y = a.y + b.y;
    
    return c;
}

fn in_range(x_min: i32, x_max: i32, y_min: i32, y_max: i32, point: &coord) -> bool {
    if point.x < x_min || point.x > x_max {
        return false;
    }
    if point.y < y_min || point.y > y_max {
        return false;
    }
    return true;
}

fn startsWith(haystack: &mut Chars, needle: &Chars) -> bool {
    let mut hCur: Chars<'_> = (*haystack).clone();
    let mut nCur: Chars<'_> = (*needle).clone();

    loop {
        let nChar: Option<char> = nCur.next();

        if nChar == None {
            *haystack = hCur;
	    return true;
	}
        
        let hChar: Option<char> = hCur.next();
               
        if hChar == None {
            return false;
        }
        else {
            if nChar.unwrap() != hChar.unwrap() {
	        return false;
	    }	
	}
    }
}

fn ParseInt(source: &mut Chars) -> Option<i64> {
    let mut s_cur: Chars<'_> = (*source).clone();

    let mut c = s_cur.next();
    let mut c_u: char = ' ';
    let mut neg: bool = false;
    let mut i: i64 = 0;

    if c.is_none() {
        return None;
    }
    c_u = c.unwrap();
    
    if c_u == '-' {
        neg = true;
        c = s_cur.next();

        if c.is_none() {
            return None;
        }
        c_u = c.unwrap();
    }

    if c_u < '0' && c_u > '9' {
        return None
    }

    let mut s_back: Chars<'_> = s_cur.clone();
    while c_u >= '0' && c_u <= '9' {
        i *= 10;
        i += c_u.to_digit(10).unwrap() as i64;
        s_back = s_cur.clone();
        c = s_cur.next();
        if c.is_none() {
            break;
        }
        c_u = c.unwrap();
    }

    *source = s_back;
    if neg {
        i *= -1;
    }
    return Some(i);
}

fn parseRange(source: &mut Chars) -> Option<(i64, i64)> {
    let mut s_cur: Chars<'_> = (*source).clone();
    let mut min_range: i64 = 0;
    let mut max_range: i64 = 0;

    let mut c = s_cur.next();

    if c.is_none() {
        return None;
    }

    let mut c_u: char = c.unwrap();
    
    if c_u != 'x' && c_u != 'y' {
        return None;
    }

    c = s_cur.next();
    if c.is_none() {
        return None;
    }
    c_u = c.unwrap();
    
    if c_u != '=' {
        return None;
    }

    let mut i = ParseInt(&mut s_cur);
    match i{
        None => {println!("Faily Waily");},
        Some(x) => {min_range = x;},
    };

    let dots = startsWith(&mut s_cur, &"..".to_string().chars());
    if dots == false {
        return None;
    }

    i = ParseInt(&mut s_cur);
    match i{
        None => {println!("Faily Waily");},
        Some(x) => {max_range = x;},
    };

    *source = s_cur;
    return Some((min_range, max_range));
}

fn relative_offsets() -> [coord; 4] {
    let offs: [coord; 4] = [coord {x:0, y:-1},
                          coord {x:-1, y:0},
                          coord {x:1, y:0},
                          coord {x:0, y:1},];
    return offs;
}

struct processed_input {
    top_left: (i64, i64),
    bottom_right: (i64, i64),
}

fn processInput(fileName : String) -> processed_input {
    let contents = fs::read_to_string(fileName).expect("Could not read file.");
    let mut i = contents.chars();
    if startsWith(&mut i, &"target area: ".to_string().chars()) {
        println!("Matched! {}", i.as_str());
    }
    else {
        println!("Not matched.");
    }

    let range = parseRange(&mut i);
    if range.is_none() {
        return processed_input {top_left: (0,0), bottom_right: (0,0)};
    }

    if !startsWith(&mut i, &", ".to_string().chars()) {
        return processed_input {top_left: (0,0), bottom_right: (0,0)};
    }

    let range2 = parseRange(&mut i);
    if range2.is_none() {
        return processed_input {top_left: (0,0), bottom_right: (0,0)};
    }

    return processed_input {top_left: (range.unwrap().0, range2.unwrap().1), bottom_right: (range.unwrap().1, range2.unwrap().0)};
}

fn findXVelocityRange(input: &processed_input) -> (i64, i64){
    let x_max: i64 = input.bottom_right.0 + 1;
    let mut x_test_min: i64 = 0;
    let mut x_test_max: i64 = x_max;

    while x_test_min != x_test_max {
        let x_test_probe: i64 = ((x_test_max-x_test_min) / 2) + x_test_min;

        let mut x_test_v: i64 = x_test_probe;
        let mut x_test_pos: i64 = 0;

        while x_test_v != 0 {
            x_test_pos += x_test_v;
            x_test_v -= 1;
        }

        if x_test_pos >= input.top_left.0 {
            x_test_max = x_test_probe;
        }
        else {
            x_test_min = x_test_probe;
            if x_test_max - x_test_min == 1 {
                x_test_min = x_test_max;
            }
        }
        
    }

    return (x_test_min, x_max);
}

fn attemptThrow(input: &processed_input, start_vel:(i64, i64)) -> (bool, i64) {
    let mut in_box: bool = false;

    let mut x_pos: i64 = 0;
    let mut x_vel: i64 = start_vel.0;
    let mut y_pos: i64 = 0;
    let mut y_vel: i64 = start_vel.1;
    let mut max_height: i64 = 0;
    
    while y_pos >= input.bottom_right.1 - 1 {
        x_pos += x_vel;
        y_pos += y_vel;
        if x_vel != 0 {
            x_vel -= 1;
        }
        y_vel -= 1;

        if y_pos > max_height {
            max_height = y_pos;
        }
        
        if x_pos >= input.top_left.0 && x_pos <= input.bottom_right.0 && y_pos >= input.bottom_right.1 && y_pos <= input.top_left.1 {
            //println!("success for x:{}, y:{}, max height:{}, launch v x:{}, y:{}", x_pos, y_pos, max_height, start_vel.0, start_vel.1);
            return (true, max_height);
        }
    }
    return (false, 0);
}

fn do_calc(input: &processed_input) {
    println!("Part 1: ");

    let xRange = findXVelocityRange(&input);
    println!("X velocity min: {}, X velocity max: {}", xRange.0, xRange.1);

    let mut max_height: i64 = 0;
    let mut max_coords: (i64, i64) = (0,0);
    let mut no_solutions: i32 = 0;
    
    println!("checking range: {}<=x<={}, {}<=y<={}", xRange.0, xRange.1, input.bottom_right.1 - 1, (input.bottom_right.1 - 1)*-1);
    for x in xRange.0..xRange.1 {
        for y in (input.bottom_right.1-1)..((input.bottom_right.1 - 1)*-1) {

            let r = attemptThrow(&input, (x, y));
            if r.0 == true {
                no_solutions += 1;
            }
            if r.0 == true && r.1 > max_height{
                max_height = r.1;
                max_coords = (x,y);
            }
        }
    }
    println!("Max height {} at launch velocities x:{} y:{}", max_height, max_coords.0, max_coords.1);
    println!("There are {} distinct launch velocities which could work.", no_solutions);
}

fn main() {
    let input = processInput("data/Day17.txt".to_string());

    println!("x {} to {}, y {} to {}", input.top_left.0, input.bottom_right.0, input.top_left.1, input.bottom_right.1);

    let test_input = processed_input {top_left: (20, -5), bottom_right: (30, -10)};
    do_calc(&input);

}
