#[allow(dead_code)]
enum Temperature{
    Celsius(i32),
    Faurenheit(i32),
}

fn main()
{
    let temp = Temperature::Faurenheit(60);
    match temp{
        Temperature::Celsius(t) if t>30 =>println!("{} is above Celsius", t),
        Temperature::Celsius(t)=>println!("{}C is equal to or below 30 Celsius",t),
        Temperature::Faurenheit(t) if t>86=>println!("{} is above 86 Fahrenheit", t),
        Temperature::Faurenheit(t)=>println!("{}F is equal or below the 86 fahrenheit",t),
    }

    let number:u8 = 5;
    match number{
        i if i ==0=>println!("zero"),
        i if i >10=>println!("greater than ten"),
        //_=>println!("rest of the number"),
        _=>println!("got below 10"),
    }
}