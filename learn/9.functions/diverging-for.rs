fn main(){
    fn sum_odd_numbers(up_to:u32)->u32{
        let mut acc =0;
        for i in 0..up_to{
            let addition: u32= match i%2==1{
                true=>i,
                false=>continue,
            };
            acc += addition;
        }
        acc
    }
    println!("sum of odd numbers upto 9: {}", sum_odd_numbers(9));
}