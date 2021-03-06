#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}


//fn rect_area() {}

fn square(a: Point, b: f32) -> Rectangle {
    Rectangle { p1: a, p2: Point { x: b, y: b } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_3_1_structs() {
        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };
        println!("{:?}", peter);

        let point = Point { x: 0.3, y: 0.4 };
        println!("point coordinates: ({}, {})", point.x, point.y);

        let new_point = Point { x: 0.1, ..point };

        println!("second point: ({}, {})", new_point.x, new_point.y);

        let Point { x: my_x, y: my_y } = point;

        let _rectangle = Rectangle { p1: Point { x: my_x, y: my_y }, p2: point };

        let _nil = Nil;

        let pair = Pair(1, 0.1);

        println!("pair contains {:?} and {:?}", pair.0, pair.1);

        let Pair(integer, decimal) = pair;

        println!("pair contains {:?} and {:?}", integer, decimal);
    }
}