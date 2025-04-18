//foreign function interface -ffi
//foreign functions must be declared inside an extern block a#
use std::fmt;
#[cfg(target_family = "windows")]
#[link(name = "msvcrt")]
extern{
    fn csqrtf(z:Complex)->Complex;
    fn ccosf(z: Complex)->Complex;
}
#[cfg(target_family ="unix")]
#[link(name = "m")]
extern {
    fn csqrtf(z: Complex)->Complex;
    fn ccosf(z: Complex)->Complex;
}
fn cos(z:Complex)->Complex{
    unsafe{ccosf(z)}
}
fn main(){
    let z = Complex{re:-1., im: 0.};
    let z_sqrt = unsafe{csqrtf(z)};
    println!("the square root of {:?} is {:?}", z, z_sqrt);
    println!("cos({:?}) = {:?}", z, cos(z));
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex{
    re:f32,
    im:f32,
}
impl fmt::Debug for Complex{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
            if self.im < 0.{
                write!(f, "{}-{}i", self.re, -self.im)
            }else{
                write!(f, "{}+{}i", self.re, self.im)
            }
    }
}