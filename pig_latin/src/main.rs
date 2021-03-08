fn main() {
}    
  


fn latinify (word: &str) -> String {

    let result = match word.chars().next() {
        Some(c) => match c {
            'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
            _=> format!("{}-{}ay", word.chars().skip(1).take(word.len()).collect::<String>(), word.chars().next().unwrap()),
        },
        None => String::new(),
    };


    result
}



#[cfg(test)]
mod tests {

    use crate::latinify;

    #[test]
    fn empty_string(){
        assert_eq!(latinify(""), "");
    }

    #[test]
    fn apple() {
        assert_eq!(latinify("apple"), "apple-hay");
    }

    #[test]
    fn first() {
        assert_eq!(latinify("first"), "irst-fay");
    }

}



