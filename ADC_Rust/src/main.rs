use std::fs;
use std::fmt::Debug;


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Line {
    p1: Point,
    p2: Point,
}

fn main() {
 //   let mut points: HashMap<Point, i32> = HashMap::new();

    let mut grid: Vec<Vec<u8>> = vec![vec![0; 1000]; 1000];
    let mut duplicates = 0;

    let lines = create_lines_from_file();

    for l in lines {
        mark_point(l, &mut grid);
    }

    for x in grid {
        for y in x{
            if y >= 2 {
                duplicates += 1;
            }
        }
    }

    println!("Duplicates {}", duplicates);


}

fn create_lines_from_file() -> Vec<Line> {
    let input = fs::read_to_string("../inputs/input_day5.txt").expect("Error");
    let split = input.split("\n");
    let vec: Vec<&str> = split.collect();

    let mut lines: Vec<Line> = Vec::new();
    //let mut vec_p: Vec<&str> = Vec::new();

    for x in vec {
        let new_string = x.replace(" -> ", ",");
        let vec_p: Vec<&str> = new_string.split(",").collect();

        if vec_p[0] == "" {     // last line
            break;
        }

        if vec_p[0] == vec_p[2] || vec_p[1] == vec_p[3] {
            let point1: Point = Point::new(vec_p[0].parse().unwrap(), vec_p[1].parse().unwrap());
            let point2: Point = Point::new(vec_p[2].parse().unwrap(), vec_p[3].parse().unwrap());

            let line: Line = Line::new(point1, point2);
            lines.push(line);
        }

    }
    lines
}

fn mark_point(line: Line, mut grid: &mut Vec<Vec<u8>>){
    if line.p1.x == line.p2.x {     // Vertical Line
        if line.p1.y < line.p2.y {
            let mut y = line.p1.y;
            for _ in line.p1.y..line.p2.y + 1 {
                grid[line.p1.x as usize][y as usize] += 1;
                y += 1;
            }
        } else {
            let mut y = line.p1.y;
            for _ in line.p2.y..line.p1.y + 1 {
                grid[line.p1.x as usize][y as usize] += 1;
                y -= 1;
            }
        }
    } else {    // Horizontal Line
        if line.p1.x < line.p2.x {
            let mut x = line.p1.x;
            for _ in line.p1.x..line.p2.x + 1 {
                grid[x as usize][line.p1.y as usize] += 1;
                x += 1;
            }
        } else {
            let mut x = line.p1.x;
            for _ in line.p2.x..line.p1.x + 1 {
                grid[x as usize][line.p1.y as usize] += 1;
                x -= 1;
            }
        }
    }
}


impl Point {
    fn new(x: i32, y: i32) -> Point {
        let point = Point {x, y};
        point
    }
}

impl Line {
    fn new(p1: Point, p2: Point) -> Line {
        let line = Line {p1, p2};
        line
    }

}
