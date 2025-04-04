#![allow(dead_code)]
#[derive(Debug)] enum Food {CordonBleu, Steak, Sushi}
#[derive(Debug)] enum Day {Monday, Tuesday, Wednesday}

fn have_ingredients(food: Food)->Option<Food>{
    match food{
        Food::Sushi=>None,
        _=>Some(food),
    }
}
fn have_recipe(food: Food)->Option<Food>{
    match food{
        Food::CordonBleu=>None,
        _=>Some(food),
    }
}
fn have_recipe1(food:Food)->Option<Food>{
    match have_recipe(food){
        None=>None,
        Some(food)=>have_ingredients(food),
    }
}
fn cookable_v3(food:Food)->Option<Food>{
    have_recipe(food).and_then(have_ingredients)
}
fn cookable_v2(food:Food)->Option<Food>
{
    have_recipe(food).map(have_ingredients).flatten()
}
fn eat(food: Food, day:Day){
    match cookable_v3(food){
        Some(food)=>println!("yay! On {:?} we get to eat {:?}.", day, food),
        None=>println!("Oh no. We dont get to eat on {:?}", day),
    }
}
fn main(){
    let (cordon_blue, steak, sushi)=(Food::CordonBleu, Food::Steak, Food::Sushi);
    eat(cordon_blue, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}