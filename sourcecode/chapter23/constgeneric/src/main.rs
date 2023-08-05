// #![feature(const_generics)] 
#![feature(generic_const_exprs)]
// #![feature(adt_const_params)]
#![allow(incomplete_features)] 

use num::Float; 
struct Array2<T: Float, const WIDTH: usize, const HEIGHT: usize> { 
    data: [[T; WIDTH]; HEIGHT], 
} 
impl<T: Float, const WIDTH: usize, const HEIGHT: usize> Array2<T, WIDTH, HEIGHT> { 
        fn new() -> Self { 
        Self { data: [[T::zero(); WIDTH]; HEIGHT] } 
        } 
        fn width(&self) -> usize { WIDTH } 
        fn height(&self) -> usize { HEIGHT } 
} 
fn main() { 
    let matrix = Array2::<f64, 4, 3>::new(); 
    print!("{} {}", matrix.width(), matrix.height()); 
} 