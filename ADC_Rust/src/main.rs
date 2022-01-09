use std::fs;
use std::collections::HashMap;
use std::sync::Mutex;


#[macro_use]
extern crate lazy_static;



#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

lazy_static! {
    static ref ARRAY: Mutex<Vec<Point>> = Mutex::new(vec![]);
    static ref hash_map:HashMap<Point, u32> = HashMap::new();
}

fn main() {
 //   let mut points: HashMap<Point, i32> = HashMap::new();

    let mut board: HashMap<Point, u32> = HashMap::new();

    create_line_from_file(board);


}

fn create_line_from_file(board: HashMap<Point, u32>) {
    let input = fs::read_to_string("../examples/sample5.txt").expect("Error");
    let split = input.split("\n");
    let vec: Vec<&str> = split.collect();

    //let mut vec_p: Vec<&str> = Vec::new();

    for x in vec {
        let new_string = x.replace(" -> ", ",");
        let vec_p: Vec<&str> = new_string.split(",").collect();

        if vec_p[0] == vec_p[2] || vec_p[1] == vec_p[3] {
            let point1: Point = Point::new(vec_p[0].parse().unwrap(), vec_p[1].parse().unwrap());
            let point2: Point = Point::new(vec_p[2].parse().unwrap(), vec_p[3].parse().unwrap());

            let line: Line = Line::new(point1, point2);
            mark_points(line);

        }

    }

}

fn mark_points(line: Line){
    //let mut hash_map:HashMap<Point, u32> = HashMap::new();

    if line.p1.x == line.p2.x {     // Vertical Line
        println!("Line: {:?}", line);
        if line.p1.y < line.p2.y {
            let mut y = line.p1.y;
            for _ in line.p1.y..line.p2.y + 1 {
                ARRAY.lock().unwrap().push(Point{x: line.p1.x, y: y,});
                hash_map.insert(Point{x: line.p1.x, y: y,}, 0);
                println!(" Marked Point x: {}, y: {}", line.p1.x, y);
                y += 1;
            }
        } else {
            let mut y = line.p1.y;
            for _ in line.p2.y..line.p1.y + 1 {
                ARRAY.lock().unwrap().push(Point{x: line.p1.x, y: y,});
                hash_map.insert(Point{x: line.p1.x, y: y,}, 0);
                println!(" Marked Point x: {}, y: {}", line.p1.x, y);
                y -= 1;
            }
        }
    } else {    // Horizontal Line
        println!("Line: {:?}", line);
        if line.p1.x < line.p2.x {
            let mut x = line.p1.x;
            for _ in line.p1.x..line.p2.x + 1 {
                ARRAY.lock().unwrap().push(Point{x: x, y: line.p1.y,});
                hash_map.insert(Point{x: x, y: line.p1.y,}, 0);
                println!(" Marked Point x: {}, y: {}", x, line.p1.y);
                x += 1;
            }
        } else {
            let mut x = line.p1.x;
            for _ in line.p2.x..line.p1.x + 1 {
                ARRAY.lock().unwrap().push(Point{x: x, y: line.p1.y,});
                hash_map.insert(Point{x: x, y: line.p1.y,}, 0);
                println!(" Marked Point x: {}, y: {}", x, line.p1.y);
                x -= 1;
            }
        }
    }
    println!("{:?}", ARRAY.lock().unwrap());
    println!("HASH MAP{:?}", hash_map);
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
