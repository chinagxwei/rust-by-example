trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct SuperStudent {
    name: String,
    university: String,
    git_username: String,
    fav_language: String,
}

impl Student for SuperStudent {
    fn university(&self) -> String {
        self.university.to_owned()
    }
}

impl Person for SuperStudent {
    fn name(&self) -> String {
        self.name.to_owned()
    }
}

impl CompSciStudent for SuperStudent {
    fn git_username(&self) -> String {
        self.git_username.to_owned()
    }
}

impl Programmer for SuperStudent {
    fn fav_language(&self) -> String {
        self.fav_language.to_owned()
    }
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My Git username is {}",
        student.name(),
        student.university(),
        student.git_username()
    )
}

fn example_16_8_supertraits() {
    let student = SuperStudent {
        name: "wayne".to_string(),
        university: "GX".to_string(),
        git_username: "wei".to_string(),
        fav_language: "Rust".to_string(),
    };

    println!("{}", comp_sci_student_greeting(&student));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_16_8_supertraits() {
        example_16_8_supertraits();
    }
}