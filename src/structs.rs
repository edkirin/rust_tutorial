use std::fmt;


struct User {
    first_name: String,
    last_name: String,
    email: String,
    active: bool,
}

impl User {
    fn format_name(&self) -> String {
        format!("{} {}", &self.first_name, &self.last_name)
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "{} ({}) - {}",
            self.format_name(),
            self.email,
            if self.active { "active" } else { "inactive" },
        )
    }
}

fn create_users() {
    let mut users = Vec::<User>::with_capacity(10);

    users.push(User {
        first_name: String::from("Pero"),
        last_name: String::from("Perić"),
        email: String::from("pero@example.com"),
        active: true,
    });
    users.push(User {
        first_name: String::from("Mirko"),
        last_name: String::from("Mirković"),
        email: String::from("mirko@example.com"),
        active: false,
    });

    for i in 0..users.len() {
        users[i].last_name = users[i].last_name.clone().to_uppercase();
    }

    for user in &users {
        println!("create_users: {}", user);
    }
}

pub fn structs_demo() {
    create_users();
}
