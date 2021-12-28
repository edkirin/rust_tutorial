#[derive(Debug)]
struct GenericStruct<T> {
    a: T,
    b: T,
}

// trait Format {
//     fn format(&self, caption: &str) -> String;
// }
//
// impl<T> Format for GenericStruct<T> {
//     fn format(&self, caption: &str) -> String {
//         format!("**{}** {} {}", caption, &self.a, &self.b)
//     }
// }

impl GenericStruct<i32> {
    fn add(&self) -> i32 { &self.a + &self.b }
}

impl GenericStruct<f32> {
    fn add(&self) -> f32 { &self.a + &self.b }
}

impl GenericStruct<String> {
    fn add(&self) -> String { format!("{} {}", &self.a, &self.b) }
}


pub fn generics_demo() {
    let int_val = GenericStruct::<i32> {
        a: 3,
        b: 4,
    };

    let float_val = GenericStruct::<f32> {
        a: 3.45,
        b: 4.56,
    };

    let string_val = GenericStruct::<String> {
        a: "Variable a".to_string(),
        b: "Variable b".to_string(),
    };

    println!(
        "int_val ** a: {}, b: {}, a + b = {}",
        int_val.a,
        int_val.b,
        int_val.add()
    );
    println!(
        "float_val ** a: {}, b: {}, a + b = {}",
        float_val.a,
        float_val.b,
        float_val.add()
    );
    println!(
        "string_val ** a: {}, b: {}, a + b = {}",
        string_val.a,
        string_val.b,
        string_val.add()
    );
}
