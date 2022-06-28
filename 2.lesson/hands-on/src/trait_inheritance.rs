trait Show{
    fn show(&self) -> String;
}

impl Show for i32{
    fn show(&self) -> String {
        format!("four-bytes: {}", &self)
    }
}

impl Show for f64{
    fn show(&self) -> String {
        format!("eight bytes: {}", &self)
    }
}


pub fn trait_inheritance(){
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("Answe show {}", s1);
    println!("Maybe_pi show {}", s2);
}
