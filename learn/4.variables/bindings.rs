fn main()
{
    let an_integer = 1u32;
    let a_boolean = true;

    let unit=();

    let copied_integer = an_integer;

    println!("an integer: {:?}", copied_integer);
    println!("a boolena: {:?}", a_boolean);
    println!("meet the unit value:{:?}", unit);

    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32;
}