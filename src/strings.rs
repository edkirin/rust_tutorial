fn composition() {
    let static_str = "Hello";
    let mut s = String::from(static_str);
    s.push(',');
    s.push(' ');
    s.push_str("world!");
    println!("composition: {}", s);
}

fn str_copy() {
    let mut s1 = String::from("Hello");
    let mut s2 = s1.clone();
    s1.push_str(", World!");
    s2.push_str(", Bob!");

    println!("str_copy s1: {}", s1);
    println!("str_copy s2: {}", s2);
}

fn change_str_reference() {
    fn greet(s: &mut String) {
        s.push_str(", World!");
    }

    let mut s1 = String::from("Hello");
    greet(&mut s1);

    println!("change_str_reference s1: {}", s1);
}


struct User {
    first_name: String,
    last_name: String,
}

fn string_mut() {
    let mut users: Vec<User> = vec![];
    users.push(User {
        first_name: "Pero".to_string(),
        last_name: "Perić".to_string(),
    });
    users.push(User {
        first_name: "Mirko".to_string(),
        last_name: "Mirković".to_string(),
    });

    fn upper_first_name(users: &mut Vec<User>) {
        for user in users.iter_mut() {
            user.first_name = user.first_name.to_uppercase();
        }
    }

    for user in users.iter() {
        println!("string_mut BEFORE: {} {}", user.first_name, user.last_name);
    }

    upper_first_name(&mut users);

    for user in users.iter() {
        println!("string_mut AFTER: {} {}", user.first_name, user.last_name);
    }
}


pub fn strings_demo() {
    composition();
    str_copy();
    change_str_reference();
    string_mut();
}
