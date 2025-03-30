pub trait Iterator {
    type Item;
    fn any<F>(&mut self, f:F)->bool where
        F:FnMut(Self::Item)->bool;
}

fn main()
{
    let vec1=vec![1,2,3];
    let vec2=vec![4,5,6];
//iter needs destructuring so & used
    println!("2 in vec1:{}", vec1.iter().any(|&x|x==2));
    //with out destructuring use into_iter()
    println!("2 in vec1:{}", vec2.into_iter().any(|x|x==2));

    //iter() only borrows vec1 so can use again
    println!("vec1 len:{}", vec1.len());
    println!("first element of vec1 is:{}", vec1[0]);

    //into_iter moves vec2 and its elements, can use
    //println!("first elemment of vec2 is:{}", vec2[0]);
    //println!("vec2 len:{}",vec2.len());

    let array1 = [1,2,3];
    let array2 = [4,5,6];

    println!("2 in array1:{}", array1.iter().any(|&x|x==2));
    println!("2 in array2:{}", array2.into_iter().any(|x|*x==2));


}