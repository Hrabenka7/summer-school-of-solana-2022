pub fn option_2() {
    fn might_print(option: Option<&str>) {
        match option {
            Some(text) => println!("The argument contains the following string: {}", text),
            None => println!("The argument contains None"),
        }
    }

    let something: Option<&str> = Some("Some string is inside");
    let nothing: Option<&str> = None;
    might_print(something);
    might_print(nothing);
}
