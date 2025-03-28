#[allow(dead_code)]
enum Color{
    Red,
    Blue,
    Green,
    RGB(u32,u32,u32),
    HSV(u32,u32,u32),
    HSL(u32,u32,u32),
    CMY(u32,u32,u32),
    CMYK(u32,u32,u32,u32),
}

fn main()
{
    let color = Color::CMY(122,17,40);
    println!("what color is it?");

    match color{
        Color::Blue=>println!("color is Blue"),
        Color::Red=>println!("color is Red"),
        Color::Green=>println!("color is green"),
        Color::RGB(r,g,b)=>println!("Red:{}, green:{}, blue:{}", r,g,b),
        Color::HSV(h,s,v)=>println!("Hue:{}, saturation:{}, value:{}", h,s,v),
        Color::HSL(h,s,l)=>println!("Hue:{}, saturation:{}, lightness:{}", h,s,l),
        Color::CMY(c,m,y)=>println!("Cyan:{}, magenta:{}, yellow:{}",c,m,y),
        Color::CMYK(c,m,y,k)=>println!("Cyan:{}, magenta:{}, yellow:{}, key(black):{}", c,m,y,k),
    }
}