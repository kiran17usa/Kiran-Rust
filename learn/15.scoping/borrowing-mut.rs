#[allow(dead_code)]
#[derive(Clone,Copy)]
struct Book{
    author: &'static str,
    title: &'static str,
    year: u32,
}
fn borrow_book(book:& Book){ //ref to a book
    
    println!("I mutably borrowed {} - {}edition",book.title, book.year );
}
fn new_edition(book:&mut Book){ //ref to a mutual book
    book.year = 2014; //we can read/write access due to mut
    println!("I mutably borrowed {} -{}edition", book.title, book.year);
}
fn main()
{
    let immutabook = Book{
        author: "Douglas Hosfstadter",
        title:"Godel, Escher, Bach",
        year: 1979,
    };
    let mut mutabook = immutabook;
    //borrow an immutble object
    borrow_book(&immutabook);
    //borrow a mutable object
    borrow_book(&mutabook);
    //borrow a mutable object as mutable
    new_edition(&mut mutabook);
    //cant borrow an immutable object as mutable
    //new_edition(&mut immutabook);
}