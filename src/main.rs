use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    env_logger::init();
    let user = User {
        id: 1,
        name: "Tyler".to_string(),
    };
    info!("User info: {:?}", user);
    println!("Hello, Rust!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_name() {
        let u = User {
            id: 2,
            name: "Alice".to_string(),
        };
        assert_eq!(u.name, "Alice");
    }
}
