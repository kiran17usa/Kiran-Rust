enum VeryVerboseEnumOfThingsToDoWithNumbers{
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers{
    fn run(&self, x:i32, y:i32)->i32{
        match self{
            VeryVerboseEnumOfThingsToDoWithNumbers::Add=>x+y,
            VeryVerboseEnumOfThingsToDoWithNumbers::Subtract=>x-y,
        }
    }
}
fn main()
{
    let x = Operations::Add;   
    let y=Operations::Subtract;
}