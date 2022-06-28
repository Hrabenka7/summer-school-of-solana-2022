struct Person {
    first_name: String,
    last_name: String,
}

pub fn person_impl() {
    let p: Person = Person {
        first_name: "Marketa".to_string(),
        last_name: "Skoloudikova".to_string(),
    };

    println!("Person {} {}", p.first_name, p.last_name);
}

