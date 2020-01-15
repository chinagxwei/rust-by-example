#![allow(dead_code)]

enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn example_8_5_1_2_destructure_enum() {
    let color = Color::RGB(133, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {},magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_8_5_1_2_destructure_enum() {
        example_8_5_1_2_destructure_enum()
    }
}

