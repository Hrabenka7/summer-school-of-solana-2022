struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
}

pub fn person_impl2() {
    let p = Person::new("Marketa", "Skoloudikova");
    println!("Person {} {}", p.first_name, p.last_name);
}
