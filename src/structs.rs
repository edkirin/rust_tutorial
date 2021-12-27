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
    let mut users: Vec<User> = Vec::<User>::new();
    // we can use this shorter version with vec! macro
    // let mut users: Vec<User> = vec![];

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

    // iterate through vector using index and uppercase last name
    for i in 0..users.len() {
        users[i].last_name = users[i].last_name.clone().to_uppercase();
    }

    // iterate with iter_mut and toggle user activity
    for user in users.iter_mut() {
        user.active = !user.active;
    }

    for user in users {
        println!("create_users: {}", user);
    }
}


fn mut_struct() {
    fn change_struct(user: &mut User, new_name: String) {
        user.first_name = new_name;
    }

    let mut user = User {
        first_name: String::from("Mirko"),
        last_name: String::from("Mirković"),
        email: String::from("mirko@example.com"),
        active: false,
    };
    println!("mut_struct: User before: {}", user);
    change_struct(&mut user, "Miroslav".to_string());
    println!("mut_struct: User after: {}", user);
}


pub fn structs_demo() {
    create_users();
    mut_struct();
}
