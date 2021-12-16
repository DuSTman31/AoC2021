
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

#[derive(Copy, Clone)]
#[derive(PartialEq)]
struct route {
    cost: i32,
    end_position: coord,
}

fn pythagoras(origin: &coord, dest: &coord) -> f64 {
    let d: f64 = ((dest.x as f64 - origin.x as f64).powi(2) + (dest.y as f64 - origin.y as f64).powi(2)).sqrt();
    return d;
}

fn manhattan(origin: &coord, dest: &coord) -> f64 {
    let d: f64 = (dest.x as f64 - origin.x as f64) + (dest.y as f64 - origin.y as f64);
    return d;
}
 
fn select_lowest_cost_route(space: &Vec<Vec<i32>>, routes: &Vec<route>, endpoint:  &coord) -> Option<route> {
    if routes.len() == 0 {
        return None;
    }
    else {
        let mut lowest_set: bool = false;
        let mut lowest_cost: f64 = 0.0;
        let mut lowest_cost_route: route = route {cost:0, end_position: coord {x:0, y:0}};
        for i in routes.iter() {
            let cost: f64 = i.cost as f64 + manhattan(&i.end_position, &endpoint);
            
            if !lowest_set {
                lowest_set = true;
                lowest_cost = cost;
                lowest_cost_route = *i;
            }
            else if cost < lowest_cost {
                lowest_cost = cost;
                lowest_cost_route = *i;
            }
        }

        return Some(lowest_cost_route);
    }
}

fn a_star(space: &Vec<Vec<i32>>, start_point: &coord, end_point: &coord) -> Option<route> {
    let mut routes: Vec<route> = Vec::new();
    routes.push(route {cost: 0, end_position: *start_point});

    let mut positions_seen: BTreeSet<coord> = BTreeSet::new();
    let mut positions_min: BTreeMap<coord, i32> = BTreeMap::new();
    let mut current_route = route {cost: 0, end_position: *start_point};

    while current_route.end_position != *end_point {
        let sr = select_lowest_cost_route(&space, &routes, &end_point);
        if sr.is_none() {
            return None;
        }
        else {
            current_route = sr.unwrap();

            for i in 0..routes.len() {
                if routes[i] == current_route {
                    routes.remove(i);
                    
                    break;
                }
            }

            let relative_offsets = relative_offsets();
            let new_positions = relative_offsets.iter()
                .map(|x| {return add_2d_coord(x, &current_route.end_position)})
                .filter(|x| {in_range(0, space[0].len() as i32 - 1, 0, space.len() as i32 - 1, x)});
            
            for n in new_positions {
                if !positions_seen.contains(&n) {
                    let cost: i32 = current_route.cost + space[n.y as usize][n.x as usize];
                    let nr: route = route {cost: cost, end_position: n};
                    routes.push(nr);
                    positions_seen.insert(n);
                    positions_min.insert(n, cost);
                  }
                else {
                    let cost: i32 = current_route.cost + space[n.y as usize][n.x as usize];
                    if cost < *positions_min.get(&n).unwrap() {
                        let nr: route = route {cost: cost, end_position: n};

                        let mut removed: bool  = true;
                        while removed {
                            removed = false;
                            for i in 0..routes.len() {
                                if routes[i].end_position == n {
                                    routes.remove(i);
                                    removed = true;
                                    break;
                                }
                            }
                        }
                        
                        routes.push(nr);

                        let mut xx = positions_min.get_mut(&n);
                        *xx.unwrap() = cost;
                    }
                    
                }
            }
        }
    }

    return Some(current_route);
}

fn part1(input: &Vec<Vec<i32>>) {
    println!("Part 1: ");

    let res = a_star(&input, &coord {x:0, y:0}, &coord {x: input[0].len() as i32 - 1, y: input.len() as i32 - 1});
    if res.is_none() {
        println!("No route found!");
    }
    else {
        println!("Lowest risk: {}", res.unwrap().cost);
    }
    
}

fn expand_source(input: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut op: Vec<Vec<i32>> = Vec::new();

    for i in 0..5 {
        for j in input.iter() {
            let mut line: Vec<i32> = Vec::new();
            for k in 0..5 {
                for l in j.iter() {
                    let mut risk: i32 = *l + i + k;
                    while risk > 9 {
                        risk -= 9;
                    }
                    line.push(risk);
                }
            }
            op.push(line);
        }
    }
    
    return op;
}

fn part2(input: &Vec<Vec<i32>>) {
    println!("Part 2: ");

    let expanded_input: Vec<Vec<i32>> = expand_source(&input);

    //for i in expanded_input.iter() {
    //    let mut s: String = String::new();
    //    for j in i.iter() {
    //        s.push_str(&j.to_string());
    //    }
    //    println!("{}", s);
    //}

    let res = a_star(&expanded_input, &coord {x:0, y:0}, &coord {x: expanded_input[0].len() as i32 - 1, y: expanded_input.len() as i32 - 1});
    if res.is_none() {
        println!("No route found!");
    }
    else {
        println!("Lowest risk: {}", res.unwrap().cost);
    }
}

fn main() {
    let input = processInput("data/Day15.txt".to_string());
    
    part1(&input);
    part2(&input);
}
