
use std::fs;
use std::str;
use std::str::Chars;
use std::collections::BTreeSet;
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

fn relative_offsets() -> [coord; 4] {
    let offs: [coord; 4] = [coord {x:0, y:-1},
                          coord {x:-1, y:0},
                          coord {x:1, y:0},
                          coord {x:0, y:1},];
    return offs;
}

fn processInputLine(line: &str) -> Vec<i32> {
    let mut op: Vec<i32> = Vec::new();

    for c in line.chars() {
        op.push(c.to_digit(10).expect("non-decimal.") as i32);
    }
    
    return op;
}

fn processInput(fileName : String) -> Vec<Vec<i32>> {
    let mut input: Vec<Vec<i32>> = Vec::new();
    
    let contents = fs::read_to_string(fileName).expect("Could not read file.");
    
    for el in contents.lines() {
        let line_val: Vec<i32> = processInputLine(&el);
	input.push(line_val);
    }

    return input;
}

fn get_low_points(input: &Vec<Vec<i32>>) -> Vec<coord> {
    let mut lowPoints: Vec<coord> = Vec::new();
    
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let pos: coord = coord {x:x as i32, y:y as i32,};
            let offsets: [coord; 4] = relative_offsets();

            
            let ti = offsets.iter()
                .map(|x| { add_2d_coord(x, &pos)})
                .filter(|x| {in_range(0, input[y as usize].len() as i32 - 1, 0, input.len() as i32 - 1, x)});

            let mut lowSpotted: bool = false;
            
            for co in ti {
                if input[co.y as usize][co.x as usize] <= input[pos.y as usize][pos.x as usize] {
                    lowSpotted = true;
                }
            }
            if !lowSpotted {
                lowPoints.push(pos);
            }
        }
    }
    return lowPoints;
}

fn part1(input: &Vec<Vec<i32>>) {
    println!("Part 1: ");

    let lowPoints = get_low_points(input);
    let mut answer: i32 = 0;
    
    for p in lowPoints.iter() {
        answer += (input[p.y as usize][p.x as usize] + 1);
    }

    println!("answer: {}", answer);
}

fn search(pos: &coord, input: &Vec<Vec<i32>>, seen: &mut BTreeSet<coord>, min_value: i32) -> i32
{
    let coords: [coord; 4] = relative_offsets();

    if input[pos.y as usize][pos.x as usize] < min_value {
        return 0;
    }
    if input[pos.y as usize][pos.x as usize] == 9 {
        return 0;
    }

    seen.insert(*pos);

    let ti = coords.iter()
        .map(|x| {add_2d_coord(x, pos)})
        .filter(|x| {in_range(0, input[0].len() as i32 - 1, 0, input.len() as i32 - 1, x)});

    let mut area: i32 = 1;
    for i in ti {
        if !seen.contains(&i) {
            area += search(&i, input, seen, input[pos.y as usize][pos.x as usize]);
        }
    }
    return area;
}

fn part2(input: &Vec<Vec<i32>>) {
    println!("Part 2: ");

    let low_points = get_low_points(input);
    
    let mut basins: Vec<i32> = Vec::new();

    for i in low_points.iter() {
        let mut seen: BTreeSet<coord> = BTreeSet::new();
        basins.push(search(i, input, &mut seen, input[i.y as usize][i.x as usize]));
    }

    basins.sort();
    if basins.len() >= 3 {
        println!("Answer: {} x {} x {} = {}", basins[basins.len()-1], basins[basins.len()-2], basins[basins.len()-3], basins[basins.len()-1] * basins[basins.len()-2] * basins[basins.len()-3]);
    }

}

fn main() {
   let input = processInput("data/Day9.txt".to_string());

   part1(&input);
   part2(&input);
}
