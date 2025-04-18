use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
#[cfg(target_family = "unix")]
use std::os::unix;
#[cfg(target_family ="windows")]
use std::os::windows;
use std::path::Path;
//implementation of %cat path
fn cat(path: &Path)->io::Result<String>{
    let mut f = File::open(path);
    let mut s = String::new();
    match f?.read_to_string(&mut s){
        Ok(_)=>Ok(s),
        Err(e)=>Err(e),
    }
}
//the above can also be written
/*
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}*/
//implementation of % echo s
fn echo(s:&str, path: &Path)->io::Result<()>{
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}
//imple of % touch path
fn touch(path: &Path)->io::Result<()>{
    match OpenOptions::new().create(true).write(true).open(path){
        Ok(_)=>Ok(()),
        Err(e)=>Err(e),
    }
}
fn main(){
    println!("mkdir a");
    match fs::create_dir("a"){
        Err(why)=>println!("! {:?}", why.kind()),
        Ok(_)=>{},
    }
    println!("echo hello > a/b.txt");
    echo ("hello", &Path::new("a/b.txt")).unwrap_or_else(|why|{
        println!("! {:?}", why.kind());
    });
    println!("mkdir -p a/c/d");
    fs::create_dir_all("a/c/d").unwrap_or_else(|why|{
        println!("! {:?}", why.kind());
    });
    println!("touch a/c/e.txt");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why|{
        println!("! {:?}", why.kind());
    });
    println!("ln -s ../b.txt a/c/b.txt");
    #[cfg(target_family = "unix")]{
    unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why|{
        println!("! {:?}", why.kind());
    });
    }   
    #[cfg(target_family = "windows")]{
        windows::fs::symlink_file("../b.txt", "a/c/b.txt").unwrap_or_else(|why|{
            println!("! {:?}", why.to_string());
        });
    }
    println!("cat a/c/b.txt");
    match cat(&Path::new("a/c/b.txt")){
        Err(why)=>println!("! {:?}", why.kind()),
        Ok(s)=>println!("> {}",s),
    }
    println!("ls a ");
    match fs::read_dir("a"){
        Err(why)=>println!("! {:?}", why.kind()),
        Ok(paths)=>for path in paths{
            println!("> {:?}", path.unwrap().path());
        },
    }
    println!("rm a/c/e.txt");
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("rmdir a/c/d");
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}