use std::fs;
//use std::collections::HashMap;


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}


fn main() {
 //   let mut points: HashMap<Point, i32> = HashMap::new();

    create_line_from_file();

}

fn create_line_from_file() {
    let input = fs::read_to_string("../examples/sample5.txt").expect("Error");
    let split = input.split("\n");
    let vec: Vec<&str> = split.collect();

    //let mut vec_p: Vec<&str> = Vec::new();

    for x in vec {
        let new_string = x.replace(" -> ", ",");
        let vec_p: Vec<&str> = new_string.split(",").collect();

        let point1: Point = Point::new(vec_p[0].parse().unwrap(), vec_p[1].parse().unwrap());
        let point2: Point = Point::new(vec_p[2].parse().unwrap(), vec_p[3].parse().unwrap());

        println!("Point1 {:?}", point1);
        println!("Point2 {:?}", point2);
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
