use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}


impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        
        if args.len() < 3{
            return Err("Not enough arguments");
        }


        let query = args[1].clone();
        let filename = args[2].clone();
        Ok (Config {query: query, filename: filename}) 

    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let result = search(&config.query, &contents);

    for r in result.iter(){
        println!("{}", r);
    }

    Ok(())
}

fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str>{
    
    let mut result = Vec::new();

    for l in content.lines(){
        if l.contains(query){
            result.push(l);
        }
    }

    result

}    


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_line() {
        let content = "\
        testing
        tester
        last line";

        let query = "testing";

        assert_eq!(search(query, content), vec!["testing"]);
    }

}



