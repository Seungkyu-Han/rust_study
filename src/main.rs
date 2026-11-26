use std::io::stdin;

struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        let user_password = procedure();
        if user_password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}

fn main() {
    let vault = Vault {
        password: String::from("top secret"),
        treasure: String::from("Gold"),
    };

    let mut user_input = String::new();

    stdin().read_line(&mut user_input).unwrap();

    user_input = user_input.trim().to_string();

    let hack = || user_input;

    let ext = vault.unlock(hack);

    println!("{:?}", ext);
}
