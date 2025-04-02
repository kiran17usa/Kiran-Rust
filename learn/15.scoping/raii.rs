fn create_box(){
    let _box1 = Box::new(3i32);//allocates on heap
    // _box1 destroyed here  and memory gets freed
}
fn main()
{
    let _box2 = Box::new(5i32);
    {
        let _box3 = Box::new(4i32);
        //box3 destroyed here
    }
    for _ in 0u32..1000{
        create_box(); // just creates 1000 boxes- no need to free memory manually
    }
    //box2 destroyed here
}