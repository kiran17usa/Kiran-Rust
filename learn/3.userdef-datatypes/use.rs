#![allow(dead_code)]

enum Stage{
    Beginner,
    Advanced,
}
enum Role{
    Student,
    Teacher,
}

fn main()
{
    use crate::Stage::{Beginner, Advanced};
    use crate::Role::*;//automatically use all

    let stage = Beginner;
    let role = Student;

    match stage{
        Beginner=>println!("Beginner are starting their lerarning journey"),
        Advanced=>println!("Advanced learners are mastering their subjects.."),
    }

    match role{
        Student=>println!("Students are acquiring knowledge"),
        Teacher=>println!("Teachers are spreading knowledge"),
    }
}