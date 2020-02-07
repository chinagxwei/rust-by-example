mod example_16_1_derive;
mod example_16_2_dyn;
mod example_16_3_ops;
mod example_16_4_drop;
mod example_16_5_iter;
mod example_16_6_impl_trait;
mod example_16_7_clone;
mod example_16_8_supertraits;
mod example_16_9_disambiguating;

struct Sheep { naked: bool, name: &'static str }

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{} say {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool { self.naked }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name())
        } else {
            println!("{} get a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { naked: false, name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_16_trait() {
        let mut dolly: Sheep = Animal::new("Dolly");

        dolly.talk();
        dolly.shear();
        dolly.talk();
    }
}