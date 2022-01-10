use std::fs;
use std::collections::HashMap;
use std::sync::Mutex;
use std::fmt::Debug;


#[macro_use]
extern crate lazy_static;



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

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Grid {
    points: Vec<Point>,
}

lazy_static! {
    static ref ARRAY: Mutex<Vec<Point>> = Mutex::new(vec![]);
}

fn main() {
 //   let mut points: HashMap<Point, i32> = HashMap::new();

    let mut grid: Grid = Grid::new();
    let lines = create_lines_from_file();

    for l in lines {
        grid.mark_point(l);
    }

    let x = grid.points.len();
    println!("1. {:?} Len {}", grid.points, grid.points.len());
    grid.points.clear_duplicates();

    let y = grid.points.len();
    println!("2. {:?} Len {}", grid.points, grid.points.len());

    println!("Duplicates {}", x - y);


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

/*
fn mark_points(line: Line, mut grid: Grid){
    //let mut hash_map:HashMap<Point, u32> = HashMap::new();

    if line.p1.x == line.p2.x {     // Vertical Line
        println!("Line: {:?}", line);
        if line.p1.y < line.p2.y {
            let mut y = line.p1.y;
            for _ in line.p1.y..line.p2.y + 1 {
                ARRAY.lock().unwrap().push(Point{x: line.p1.x, y: y,});
                grid.mark_point(Point{x: line.p1.x, y: y,});
                println!(" Marked Point x: {}, y: {}", line.p1.x, y);
                y += 1;
            }
        } else {
            let mut y = line.p1.y;
            for _ in line.p2.y..line.p1.y + 1 {
                ARRAY.lock().unwrap().push(Point{x: line.p1.x, y: y,});
                grid.mark_point(Point{x: line.p1.x, y: y,});
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
                grid.mark_point(Point{x: x, y: line.p1.y,});
                println!(" Marked Point x: {}, y: {}", x, line.p1.y);
                x += 1;
            }
        } else {
            let mut x = line.p1.x;
            for _ in line.p2.x..line.p1.x + 1 {
                ARRAY.lock().unwrap().push(Point{x: x, y: line.p1.y,});
                grid.mark_point(Point{x: x, y: line.p1.y,});
                println!(" Marked Point x: {}, y: {}", x, line.p1.y);
                x -= 1;
            }
        }
    }
    println!("{:?}", ARRAY.lock().unwrap());
}
*/




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

impl Grid {
    fn new() -> Self {
        let p: Vec<Point> = Vec::new();
        Grid{points: p}
    }

    fn mark_point(&mut self, line: Line){
        if line.p1.x == line.p2.x {     // Vertical Line
            //println!("Line: {:?}", line);
            if line.p1.y < line.p2.y {
                let mut y = line.p1.y;
                for _ in line.p1.y..line.p2.y + 1 {
                    self.points.push(Point{x: line.p1.x, y: y,});
                   // println!(" Marked Point x: {}, y: {}", line.p1.x, y);
                    y += 1;
                }
            } else {
                let mut y = line.p1.y;
                for _ in line.p2.y..line.p1.y + 1 {
                    self.points.push(Point{x: line.p1.x, y: y,});
                   // println!(" Marked Point x: {}, y: {}", line.p1.x, y);
                    y -= 1;
                }
            }
        } else {    // Horizontal Line
            //println!("Line: {:?}", line);
            if line.p1.x < line.p2.x {
                let mut x = line.p1.x;
                for _ in line.p1.x..line.p2.x + 1 {
                    self.points.push(Point{x: x, y: line.p1.y,});
                   // println!(" Marked Point x: {}, y: {}", x, line.p1.y);
                    x += 1;
                }
            } else {
                let mut x = line.p1.x;
                for _ in line.p2.x..line.p1.x + 1 {
                    self.points.push(Point{x: x, y: line.p1.y,});
                    //println!(" Marked Point x: {}, y: {}", x, line.p1.y);
                    x -= 1;
                }
            }
        }
    }
}


  trait Dedup<T: PartialEq + Clone> {
      fn clear_duplicates(&mut self);
  }

  impl<T: PartialEq + Clone> Dedup<T> for Vec<T> {
      fn clear_duplicates(&mut self) {
          let mut already_seen = Vec::new();
          self.retain(|item| match already_seen.contains(item) {
              true => false,
              _ => {
                  already_seen.push(item.clone());
                  true
              }
          })
      }
  }
