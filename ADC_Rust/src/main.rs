use std::fs;

/*
#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,

    x2: i32,
    y2: i32,

}
*/


fn main() {
    create_line_from_file();
}

fn create_line_from_file() {
    let points = fs::read_to_string("../examples/sample5.txt").expect("Error");
    for point in points.split("->") {
        println!("Point {}", point);
    }
}
