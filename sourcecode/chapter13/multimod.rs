// 一个名为 `mod1` 的模块
mod mod1 {
  // 在模块中的项默认带有私有可见性。
  fn private_function() {
    println!("called `mod1::private_function()`");
  }

  // 使用 `pub` 修饰语来改变默认可见性。
  pub fn function() {
    println!("called `mod1::function()`");
  }

  // 在同一模块中，项可以访问其它项，即使是私有属性。
  pub fn indirect_access() {
    print!("called `mod1::indirect_access()`, that\n> ");
    private_function();
  }

  // 项也可以嵌套。
  pub mod nested {
    pub fn function() {
      println!("called `mod1::nested::function()`");
    }
    #[allow(dead_code)]
    fn private_function() {
    println!("called `mod1::nested::private_function()`");
    }
  }
  
  // 嵌套项的可见性遵循相同的规则。
  mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
      println!("called `mod1::private_nested::function()`");
    }
  }
}
fn function() {
  println!("called `function()`");
}
fn main() {
  // 模块允许在拥有相同名字的项之间消除歧义。
  function();
  mod1::function();
  // 公有项，包括内部嵌套的公有项，可以在父级的模块中访问到。
  mod1::indirect_access();
  mod1::nested::function();
  // 一个模块中的私有项不能被直接访问，即使私有项嵌套在公有的模块中：
  // 报错！`private_function` 是私有的。
  //mod1::private_function();
  // 试一试 ^ 将此行注释去掉
  // 报错！ `private_function` 是私有的。
  //mod1::nested::private_function();
  // 试一试 ^ 将此行注释去掉
  // 报错！ `private_nested` 是私有的模块。
  //mod1::private_nested::function();
  // 试一试 ^ 将此行注释去掉
}