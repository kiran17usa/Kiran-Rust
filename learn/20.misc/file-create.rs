static LOREM_IPSUM: &str = 
"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main(){
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();
    let mut file = match File::create(&path){
        Err(why)=>panic!("couldnt create {}: {}", display, why),
        Ok(file)=>file,
    };
    match file.write_all(LOREM_IPSUM.as_bytes()){
        Err(why)=>panic!("couldnt write to {}:{}", display, why),
        Ok(_)=>println!("successfully wrote to {}", display),
    }
}