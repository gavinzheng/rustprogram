mod top { 
  pub mod inner1 { 
    pub mod inner2 { 
      pub(self) fn function1() {} 
      pub(super) fn function2() {} 
      pub(crate) fn function3() {} 
    } 

    // 下面的语句会触发错误:  function1是pub（self），只适用于inner2模块，
    // 不适用于用pub use向外层重新导出
    // pub use self::inner2::function1; 

    fn caller1() { 
      // 下面的语句会触发错误 :  function1是pub（self），只适用于inner2模块，不适用于inner1模块
      // self::inner2::function1(); 
    } 
  } 
  fn caller2() { 
    // 下面的语句会触发错误 :  function2是pub(super)，适用于inner1模块，不适用于top模块 
    // self::inner1::inner2::function2(); 
  } 
} 
  // 下面的语句会触发错误 :  function3是pub(crate)，适用于本crate，不适用于用pub use向外层重新导出
  // pub use ::top::inner1::inner2::function3; 
fn main(){

}