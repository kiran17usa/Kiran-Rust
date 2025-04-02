struct Fibonanci{
    curr:u32,
    next: u32,
}
impl Iterator for Fibonanci{
    type Item = u32;
    fn next(&mut self)->Option<Self::Item>{
        let current = self.curr;
        self.curr=self.next;
        Some(current)
    }
}
fn fibonancci()->Fibonanci{
    Fibonanci{curr:0, next:1}
}
fn main(){
    let mut sequence = 0..3;
    println!("Four consecutive next calls on 0..3");
    println!(">{:?}", sequence.next());
    println!(">{:?}", sequence.next());
    println!(">{:?}", sequence.next());
    println!(">{:?}", sequence.next());

    println!("Iterate through 0..3 using for");
    for i in 0..3{
        println!("> {}", i)
    }
    println!("the first four terms of the fibonancci sequence are");
    for i in fibonancci().take(4){
        println!("> {}", i);
    }
    println!("the next four terms of the fibonancci sequence are");
    for i in fibonancci().skip(4).take(4){
        println!("> {:?}", i);
    }
    let array = [1u32, 3,3,7];
    println!("Iterate the following array{:?}", &array);
    for i in array.iter(){
        println!("> {}", i);
    }
}