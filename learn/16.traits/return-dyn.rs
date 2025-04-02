//return should be concrete to allocate memory 
//cant return Object type as it not sure how much will occupy in diff implementations

struct Sheep{}
struct Cow{}
trait Animal{
    fn noise(&self)->&'static str;
}

impl Animal for Sheep{
    fn noise(&self)->&'static str{
        "baaaaah!"
    }
}
impl Animal for Cow{
    fn noise(&self)->&'static str{
        "moooooo!"
    }
}
//return some struct that implements Animal
fn random_animal(random_number: f64)->Box<dyn Animal>{
    if random_number <0.5{
        Box::new(Sheep{})
    }else{
        Box::new(Cow{})
    }
}

fn main()
{
    let random_number = 0.534;
    let animal = random_animal(random_number);
    println!("You hv randomly chosen an animal, and it is {}", animal.noise());
}