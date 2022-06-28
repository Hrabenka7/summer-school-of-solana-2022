pub fn option() {
    // returns target text, otherwise none
    fn contains_char(text: &str, target_c: char) -> Option<&str> {
        if text.chars().any(|ch| ch == target_c) {
            return Some(text);
        } else {
            return None;
        }
    }

    let a: Option<&str> = contains_char("Winter School of Solana", 'a');
    let b: Option<&str> = contains_char("Winter School of Solana", 'b');
    println!("{:?}", a);
    println!("{:?}", b);


}