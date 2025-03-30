//functionality which requires  to make the closure work without annotation

//ref : &T,mut ref:  &mut T,by value: T

fn main()
{
    use std::mem;

    let color=String::from("green");

    let print = ||println!("color is :{}", color);
    //directly call 
    print();
    let _reborrow = &color;//can reborrow the color immutably here since its not moved yet &
    print();

    let _color_moved=color;
    let mut count=0;
    let mut inc=||{
        count+=1;
        println!("count:{}", count);
    };
    inc();
     //let _reborrow = &count; //the below still uses count hence cant borrow
    inc();

    let _count_reborrowed =  &mut count;//no borrow of count later hence we can borrow from here onwards sicne its mut
    let movable = Box::new(3);

    let consume=||{
        println!("movable:{:?}", movable);
        mem::drop(movable);
    };
    consume();

    let haystack = vec![1,2,3];
    let contains = move |needle|haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
}
