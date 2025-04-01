//outer attribute - applies to the entire structure
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//inner attribute - applied to the entire file
#![allow(unused_variables)]

fn main() {
    let x = 3; // This would normally warn about an unused variable.
}