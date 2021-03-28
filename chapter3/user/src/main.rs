fn main() {
    let user = User::new(1, String::from("alice"));
    let user_renamed = &user.change_name(String::from("bob"));

    println!("user.name: {}", user.name);
    println!("user_renamed.name: {}", user_renamed.name);
    println!("user == user_renamed: {}", user == *user_renamed);
}

struct User {
    id: u64,
    name: String,
}

impl User {
    fn new(id: u64, name: String) -> Self {
        User { id, name }
    }

    fn change_name(&self, name: String) -> Self {
        User { id: self.id, name }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
