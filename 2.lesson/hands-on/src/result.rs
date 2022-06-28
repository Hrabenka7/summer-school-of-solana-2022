pub fn result(){
    fn check_length(s: &str, min:usize) -> Result<&str,String> {
        if s.chars().count() >= min {
            return Ok(s)
        } else {
            return Err(format!("'{}' is not long enough", s));
        }
    }
    
    let func_return: Result<&str, String> = check_length("some string literal", 30);
    let a_str= match func_return{
        Ok(a_str) => a_str,
        Err(error) => panic!("Problem running 'check_length':\n {:?}", error),
    };

    println!("{}", a_str);
} 