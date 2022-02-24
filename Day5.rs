
use std::fs;
use std::str;
use std::str::Chars;
use std::collections::BTreeMap;


struct processedInput {
    lines: Vec<((i64, i64),(i64, i64))>,
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

fn process_number_pair(input: &mut Chars) -> Option<(i64, i64)> {
    let mut i_cpy: Chars = (*input).clone();

    let res = ParseInt(&mut i_cpy);
    if res.is_none() {
        return None;
    }

    let x = res.unwrap();

    let c = i_cpy.next();
    if c.is_none() {
        return None;
    }
    if c.unwrap() != ',' {
        return None;
    }

    let res2 = ParseInt(&mut i_cpy);
    if res2.is_none() {
        return None;
    }
    let y = res2.unwrap();

    *input = i_cpy;
    return Some((x,y));
}

fn processInput(fileName : String) -> processedInput {
    let mut input: Vec<i64> = Vec::new();
    
    let contents = fs::read_to_string(fileName).expect("Could not read file.");

    let mut line_descriptors: Vec<((i64, i64),(i64, i64))> = Vec::new();
    let mut lines = contents.lines();
    for l in lines {
        let mut c = l.chars();
        let s1 = process_number_pair(&mut c);
        if s1.is_none() {
            continue;
        }

        if !startsWith(&mut c, &" -> ".to_string().chars()) {
            continue;
        }

        let s2 = process_number_pair(&mut c);
        if s2.is_none() {
            continue;
        }

        let s1d = s1.unwrap();
        let s2d = s2.unwrap();
        line_descriptors.push(((s1d.0,s1d.1),(s2d.0,s2d.1)));
    }
        
    return processedInput {lines: line_descriptors};
}

fn drawLine(board: &mut Vec<Vec<i32>>, line: ((i64,i64),(i64,i64))) {
    let mut dx: i64 = 0;
    let mut dy: i64 = 0;

    let mut x: i64 = (line.0).0;
    let mut y: i64 = (line.1).1;

    if (line.1).0 > (line.0).0 {
        dx = 1;
    }
    else if (line.1).0 == (line.0).0 {
        dx = 0;
    }
    else if (line.1).0 < (line.0).0 {
        dx = -1;
    }

    if (line.1).1 > (line.0).1 {
        dy = 1;
    }
    else if (line.1).1 == (line.0).1 {
        dy = 0;
    }
    else if (line.1).1 < (line.0).1 {
        dy = -1;
    }

    let mut y_min = 0;
    let mut y_max = 0;
    if  (line.0).1 > (line.1).1 {
        y_min = (line.1).1;
        y_max = (line.0).1;
    }
    else {
        y_min = (line.0).1;
        y_max = (line.1).1;
    }

    let mut x_min: i64 = 0;
    let mut x_max: i64 = 0;
    if (line.0).0 < (line.1).0 {
        x_min = (line.0).0;
        x_max = (line.1).0;
    }
    else {
        x_min = (line.1).0;
        x_max = (line.0).0;
    }

    
    let mut x_cur = (line.0).0;
    let mut y_cur = (line.0).1;
    loop{
        board[y_cur as usize][x_cur as usize] += 1;
        y_cur += dy;
        x_cur += dx;
        if y_cur < y_min || y_cur > y_max || x_cur < x_min || x_cur > x_max {
            break;
        }
    }
    
    
}
                   
fn part1(input: &processedInput) {
    println!("Part 1: ");

    let mut maxX: i64 = 0;
    let mut maxY: i64 = 0;

    for i in input.lines.iter() {
        if (i.0).0 > maxX {
            maxX = (i.0).0;
        }
        if (i.1).0 > maxX {
            maxX = (i.1).0;
        }
        if (i.0).1 > maxY {
            maxY = (i.0).1;
        }
        if (i.1).1 > maxY {
            maxY = (i.1).1;
        }
    }

    let mut board: Vec<Vec<i32>> = Vec::new();
    for y in 0..maxY+1 {
        let mut row: Vec<i32> = Vec::new();
        for x in 0..maxX+1 {
            row.push(0);
        }
        board.push(row);
    }

    for l in input.lines.iter().filter(|x| {if (x.0).0 == (x.1).0 || (x.0).1 == (x.1).1 { return true;} else { return  false;}}) {
        //println!("{},{}   {},{}", (l.0).0, (l.0).1, (l.1).0, (l.1).1);

        drawLine(&mut board, *l);

    }

    let mut multi_points: i32 = 0;

    for i in board.iter() {
        for j in i.iter() {
            if *j > 1 {
                multi_points += 1;
            }
        }
    }

    println!("{} multi points.", multi_points);
}
                   
                 
fn part2(input: &processedInput) {
    println!("Part 2: ");

    let mut maxX: i64 = 0;
    let mut maxY: i64 = 0;

    for i in input.lines.iter() {
        if (i.0).0 > maxX {
            maxX = (i.0).0;
        }
        if (i.1).0 > maxX {
            maxX = (i.1).0;
        }
        if (i.0).1 > maxY {
            maxY = (i.0).1;
        }
        if (i.1).1 > maxY {
            maxY = (i.1).1;
        }
    }

    let mut board: Vec<Vec<i32>> = Vec::new();
    for y in 0..maxY+1 {
        let mut row: Vec<i32> = Vec::new();
        for x in 0..maxX+1 {
            row.push(0);
        }
        board.push(row);
    }

    for l in input.lines.iter() {
        //println!("{},{}   {},{}", (l.0).0, (l.0).1, (l.1).0, (l.1).1);

        drawLine(&mut board, *l);

    }

    let mut multi_points: i32 = 0;

    for i in board.iter() {
        for j in i.iter() {
            if *j > 1 {
                multi_points += 1;
            }
        }
    }

    println!("{} multi points.", multi_points);
}

fn main() {
    let input = processInput("data/Day5.txt".to_string());


    
    
    part1(&input);
    part2(&input);
}
