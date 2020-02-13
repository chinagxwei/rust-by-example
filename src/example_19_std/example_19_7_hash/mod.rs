mod example_19_7_1_alt_key_types;
mod example_19_7_2_hashset;


fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed. Please hang up and try again.",
        "645-7689" => "Hello, this is Mr.Awesome's Pizza. My name is Fred.",
        _ => "Hi! Who is this again?"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn example_19_7_hash() {
        let mut contacts = HashMap::new();

        contacts.insert("Daniel", "798-1364");
        contacts.insert("Ashley", "645-7689");
        contacts.insert("Katie", "435-8291");
        contacts.insert("Robert", "956-1745");

        match contacts.get("Daniel") {
            Some(&number) => println!("Calling Daniel: {}", call(number)),
            _ => println!("Don't have Daniel's number.")
        }

        contacts.insert("Daniel", "164-6743");

        match contacts.get("Ashley") {
            Some(&number) => println!("Calling Ashley: {}", call(number)),
            _ => println!("Don't have Ashley's number.")
        }

        contacts.remove("Ashley");

        for (&k, &v) in contacts.iter() {
            println!("Calling {}: {}", k, call(v));
        }
    }
}