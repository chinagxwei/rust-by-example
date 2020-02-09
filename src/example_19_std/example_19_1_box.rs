#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn box_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_19_1_box() {
        let point = origin();

        let rectangle = Rectangle { p1: origin(), p2: Point { x: 3.0, y: 4.0 } };

        let boxed_rectangle = Box::new(
            Rectangle { p1: origin(), p2: origin() }
        );

        let boxed_point = Box::new(origin());

        let box_in_a_box = Box::new(box_origin());

        println!("Point occupies {} bytes in the stack", std::mem::size_of_val(&point));

        println!("Rectangle occupies {} bytes in the stack", std::mem::size_of_val(&rectangle));

        println!("Boxed point occupies {} bytes in the stack", std::mem::size_of_val(&boxed_point));
        println!("Boxed rectangle occupies {} bytes in the stack", std::mem::size_of_val(&boxed_rectangle));
        println!("Boxed box occupies {} bytes in the stack", std::mem::size_of_val(&box_in_a_box));

        let unboxed_point = *boxed_point;

        println!("Unboxed point occupies {} bytes in the stack", std::mem::size_of_val(&unboxed_point));
    }
}