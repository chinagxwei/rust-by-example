struct Sheep;

struct Cow;

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 { Box::new(Sheep) } else { Box::new(Cow) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_16_2_dyn() {
        let random_number = 0.234;
        let animal = random_animal(random_number);

        println!("You've randomly chosen an animal, and it says {}", animal.noise());
    }
}