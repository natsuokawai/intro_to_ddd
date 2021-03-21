use std::fmt;
use regex::Regex;

fn main() {
    let name = FullName::new(String::from("Natsuo"), String::from("Kawai"));
    println!("name.first_name: {}", name.first_name);
    println!("name.last_name : {}", name.last_name);

    let name2 = FullName::new(String::from("Natsuo"), String::from("Kawai"));
    println!("name2.first_name: {}", name2.first_name);
    println!("name2.last_name : {}", name2.last_name);

    println!("name == name2: {}", name == name2);
}

#[derive(PartialEq)]
struct FullName {
    first_name: String,
    last_name: String,
}

impl fmt::Display for FullName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

impl FullName {
    fn new(first_name: String, last_name: String) -> Self {
        FullName::validate_name(&first_name);
        FullName::validate_name(&last_name);

        FullName {
            first_name,
            last_name,
        }
    }

    fn validate_name(name: &str) {
        let re = Regex::new(r"^[a-zA-Z]+$").unwrap();
        if !re.is_match(name) {
            panic!("name contains invalid characters.");
        }
    }
}
