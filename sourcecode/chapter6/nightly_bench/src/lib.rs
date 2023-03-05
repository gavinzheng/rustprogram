// bench_example/src/lib.rs

#![feature(test)] 					// 必须要置于模块顶部
extern crate test; 					// 导入test crate
use test::Bencher; 

pub fn func_slowly() { 
    print!("#"); 
    for _ in 1..20_000_000 {}; 
}
pub fn func_fast() { 
}

#[bench] 							// 标记要测试的函数
fn bench_func_slowly(b: &mut Bencher) { 
    b.iter(|| func_slowly()); 
}

#[bench] 
fn bench_func_fast(b: &mut Bencher) { 
    b.iter(|| func_fast()); 
} 
