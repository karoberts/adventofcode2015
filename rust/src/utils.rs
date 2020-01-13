use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// allows for concise hashmap construction
/// like this: hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

/// read a file and return an iterator of lines
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, 
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// converts a regular expression capture to string and parses it, panics if there is a problem
pub fn cap_to<T: std::str::FromStr>(cap: Option<regex::Match>) -> T 
{
    match cap.map(|m| m.as_str().parse::<T>() ) {
        Some(s) => match s {
            Ok(ss) => ss,
            Err(_) => panic!("failed to parse {:?}", cap)
        },
        None => panic!("failed to get value {:?}", cap)
    }
}