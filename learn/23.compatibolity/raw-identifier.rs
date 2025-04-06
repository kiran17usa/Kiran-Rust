/*
extern crate foo;

fn main() {
    foo::try();
}
*/
//above will fail

extern crate foo;

fn main() {
    foo::r#try();
}