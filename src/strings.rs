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

pub fn strings_demo() {
    composition();
    str_copy();
    change_str_reference();
}
