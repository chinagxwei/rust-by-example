#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    CordonBleu,
    Steak,
    Sushi,
}

#[derive(Debug)]
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food)
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food)
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_ingredients(food) {
        Some(food) => match have_recipe(food) {
            Some(food) => Some(food),
            None => None
        },
        None => None
    }
}

fn cookable_v2(food: Food) -> Option<Food> {
    have_ingredients(food).and_then(have_recipe)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_2_2_and_then() {
        let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

        eat(cordon_bleu, Day::Sunday);
        eat(steak, Day::Monday);
        eat(sushi, Day::Tuesday);
    }
}