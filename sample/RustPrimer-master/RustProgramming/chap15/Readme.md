# 第十五章 智能指针

- 简介
  - 引用：最常见的指针类型，不拥有资源的所有权
  - 智能指针：一类表现类似指针的数据结构
    - 通常拥有资源的所有权
    - 通常实现了`Deref`和`Drop`特性
    - 常用的智能指针
      - `Box<T>` 用于在堆上分配值
      - `Rc<T>` 一个引用计数类型，其数据可以有多个所有者
      - `Ref<T>` `RefMut<T>` 和 `RefCell<T>`
- `Box<T>`
  - 用途
    1. 在需要确定大小的上下文中使用大小未知类型（如递归类型），本节讨论这种情况
    2. 需要在不复制数据的情况下转移大量数据的所有权时。转移栈上数据的所有权将导致数据复制，对于大量数据 ，复制时间可能很长，这时候可以使用`Box<T>`：转移`Box<T>`的所有权只需要复制栈上的数据指针，而不需要复制成堆上的数据，所以开销很小。
    3. 需要一个实现了特定trait的不确定类型时，这就是第17章将讨论的特性对象（trait object）
  - 不使用`Box<T>`创建递归类型
    - 下述代码无法通过编译，提示递归类型List具有无限大小，建议使用`box`、`Rc`或者`&`实现的间接引用
    - 非递归的枚举类型的大小，等于最大字段的大小

      ```rust
      enum List {
        Cons(i32, List),
        Nil,
      }

      fn main() {
        let list = Cons(1, Cons(2, Cons(3, Nil)));
      }
      ```

  - 使用`Box<T>`创建递归类型
    - `Box<T>`提供了间接存储和堆分配，没有其他特殊功能，所以没有其他智能指针的额外开销
    - `Box<T>`实现了`Deref`特性，所以是智能指针，可以被当做引用对待。
    - 离开作用域时，`Box<T>`实现的`Drop`特性中的`drop`方法被调用，这个方法会清理堆上的数据（栈上的指针被自动清理）。

      ```rust
      enum List {
        Cons(i32, Box<List>),SSS
        Nil,
      }

      fn main() {
        let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
      }
      ```

- `Deref`特性
  - 解引用
    - 引用可看做是指针，可通过解引用运算符 `*` (星号)访问指针指向的值
    - `Box<T>` 实现了`Deref`特性，可以被当做指针使用，即也可以对`Box<T>`类型使用解引用运算符

      ```rust
      let x = 5;
      let y = &x;
      let z = Box::new(x);

      assert_eq!(x,5);
      assert_eq!(*y,5);
      assert_eq!(*z,5);
      ```

  - 实现`Deref`特性
    - 自定义类型`MyBox<T>`实现`std::ops::Deref`特性后，就可以对其示例使用解引用运算符了
    - 对实现了`std::ops::Deref`的类型实例使用解引用时，实际上相当于`*(z.deref())`，所以`deref()`方法必须返回引用值：如果返回的不是引用值，则不能对其使用解引用运算符

      ```rust
      impl<T> MyBox<T>{
        fn new(x: T) -> MyBox<T>{
          MyBox(x)
        }
      }

      use std::ops::Deref;

      impl<T> Deref for MyBox<T>{
        type Target = T;
        fn deref(&self) -> &T{
          &self.0
        }
      }

      let x = 5;
      let y = &x;
      let z = MyBox::new(x);

      assert_eq!(x,5);
      assert_eq!(*y,5);
      assert_eq!(*z,5);
      ```

  - 解引用强制多态（deref coercions）
    - 传递函数/方法参数时，Rust会在必要时多次调用Deref特性的deref方法，使得传入的参数类型与函数/方法要求的类型一致，这就是解引用强制多态(deref coercions)
    - 解引用强制多态发生在编译时，没有运行时性能开销
    - 解引用强制多态规则
      - 当`T: Deref<Target=U>`时从`&T`到`&U`
      - 当`T: DerefMut<Target=U>`时从`&mut T`到`&mut U`
      - 当`T: Deref<Target=U>`时从`&mut T`到`&U`
        ```rust
        fn hello(x : &str){ println!("{}",x); };
        let a = MyBox::new(String::from("ABC"));
        hello(a.deref().deref());
        hello(&a);
        hello(&*a);
        //下面两句是错误的
        //hello(*a);
        //hello(a);
        ```

- `Drop`特性
  - 实现Drop特性
    ```rust
    struct CustomSmartPointer {
      data: String,
    }

    impl Drop for CustomSmartPointer {
      fn drop(&mut self) {
          println!("Dropping CustomSmartPointer with data `{}`!", self.data);
      }
    }

    fn main() {
      let c = CustomSmartPointer { data: String::from("my stuff") };
      let d = CustomSmartPointer { data: String::from("other stuff") };
      println!("CustomSmartPointers created.");
    }
    ```

  - 手动调用drop方法
    - 变量离开作用域时，drop方法被自动调用，这个特性不能关闭
    - 不能手动调用变量的drop方法
    - 但可以调用std::mem::drop函数，传入变量，实现手动析构
- `Rc<T>` 引用计数的智能指针
  - 使用场景：单线程、多处只读使用、无法确定哪部分最后结束使用（然后释放资源）
    - 希望在堆上分配一些内存供程序的多个部分读取，但无法在编译时确定程序的哪一部分会最后结束使用
    - 如果知道哪部分会最后结束使用，则让这部分成为数据的所有者，让其离开作用域时释放数据就可以了；其他部分使用数据的不可变引用就可以了
  - 示例：不使用`Rc<T>`

    ```rust
    enum List {
      Cons(i32, Box<List>),
      Nil,
    }
    let a = List::Cons(5,Box::new(List::Cons(10,Box::new(List::Nil))));
    let _b = List::Cons(3, Box::new(a));
    //编译错误：上个语句将变量a的所有权移动到b中了,a已经失效
    //let c = List::Cons(4, Box::new(a));
    ```

  - 示例：使用`Rc<T>`
    - `Rc::clone(&a)`的写法
      - 下述代码中的`Rc::clone(&a)`也可以写成`a.clone()`，但是Rust中的习惯写法是`Rc::clone(&a)`
      - `Rc::clone(&a)`强调了通过`Rc`进行克隆，只增加引用计数，而不进行深拷贝
      - `a.clone()`这种写法通常用于深拷贝的情况
    - `Rc::strong_count(&a)`获取引用计数、克隆会增加引用计数

      ```rust
      use std::rc::Rc;
      enum List {
        Cons(i32, Rc<List>),
        Nil,
      }

      let a = Rc::new(List::Cons(5,Rc::new(List::Cons(10,Rc::new(List::Nil)))));
      println!("a的引用计数：{}",Rc::strong_count(&a));//1

      let _b = List::Cons(3, Rc::clone(&a));
      println!("a的引用计数：{}",Rc::strong_count(&a));//2

      {
      let _c = List::Cons(4, Rc::clone(&a));
      println!("a的引用计数：{}",Rc::strong_count(&a));//3
      }

      println!("a的引用计数：{}",Rc::strong_count(&a));//2
      ```

- `RefCell<T>`与内部可变性
  - 对于引用和`Box<T>`，编译时进行借用规则检查，检查不通过时发生编译错误。这是大多数情况下的合理选择，但过于保守，除了能检查出错误的程序外，有时候还会拒绝正确的程序。
  - 对于`RefCell<T>`，程序运行时进行借用规则检查，检查不通过则发生`panic!`。`RefCell<T>`实现了内部可变性设计(interior mutability)模式，允许通过不可变借用来修改数据。
  - 示例程序：使用`RefCell<T>`，可通过不可变引用获取内部字段的可变引用

    ```rust
    use std::cell::RefCell;

    pub trait Messager {
      fn send(&self, msg: &str);
    }

    struct MockMessenger {
      sent_messages: RefCell<Vec<String>>,
      //sent_messages: Vec<String>,
    }

    impl MockMessenger {
      fn new() -> MockMessenger {
        MockMessenger { sent_messages: RefCell::new(vec![]) }
      }

      // 编译通过，运行时panic!
      fn demo(&self) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();
        one_borrow.push(String::from("abc"));
        two_borrow.push(String::from("123"));
      }
    }

    impl Messager for MockMessenger {
      fn send(&self, message: &str) {
        // 错误：没法通过不可变的self引用来取得self.sent_messages的可变引用(push方法要求可变引用)
        //self.sent_messages.push(String::from(message));
        self.sent_messages.borrow_mut().push(String::from(message));
      }
    }
    ```

  - 要点
    1. `use std::cell::RefCell`
    2. 声明：`sent_messages: RefCell<Vec<String>>`
    3. 创建：`sent_messages: RefCell::new(vec![])`
    4. 使用：`sent_messages.borrow_mut().push(String::from("abc"))`
    - 运行时进行借用规则检查
      - 下述代码编译通过，但运行时试图获取第二个可变引用时触发panic!
        ```rust
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();
        ```
- 对比
  - `Rc<T>`让数据有多个所有者；`Box<T>`和`RefCell<T>`让数据有单一所有者。
  - 引用和`Box<T>`在编译时执行可变/不可变借用检查；`Rc<T>`在编译时执行不可变借用检查；`RefCell<T>`在运行时执行不可变/可变借用检查。
  - `RefCell<T>`可在自身不可变的情况下，修改内部包含的值。
  - 可以用`Rc<RefCell<T>>`来获取数据的多个可变借用
- 引用循环与内存泄漏
  - 编译器不能阻止引用循环和内存泄漏

    ```rust
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    enum List {
      Cons(i32, RefCell<Rc<List>>),
      Nil,
    }

    impl List {
      fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
          Cons(_, ref item) => Some(item),
          Nil => None,
        }
      }
    }

    use self::List::{Nil,Cons};

    pub fn demo() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle; it will soverflow the stack
        //println!("a next item = {:?}", a.tail());
    }
    ```

  - 避免引用循环：将`Rc<T>`变为`Weak<T>`
    - `Rc::clone`会增加Rc实例的`strong_count`，拥有所有权的Rc实例离开作用域时使得`strong_count`减1，若`strong_count`变成零，则清理引用的资源
    - 通过`Rc::downgrade`方法得到不代表所有权关系的弱引用指针，即`Weak<T>`类型的智能指针。`Weak<T>`对应`weak_count`，`weak_count`为零不会导致清理引用的资源。
    - 调用`Weak<T>`实例的`upgrade`可得到`Option<T>`，若引用的资源还没有被清理，则得到`Some`；反之得到`None`。
        ```rust
        #[derive(Debug)]
        struct Node {
          value: i32,
          parent: RefCell<Weak<Node>>,
          children: RefCell<Vec<Rc<Node>>>,
        }

        pub fn weak_ref_demo() {
          let leaf = Rc::new(
            Node {
              value: 100,
              parent: RefCell::new(Weak::new()),
              children: RefCell::new(vec![]),
            });
          // None,1,0
          println!("leaf: value={} parent={:?} 强引用数={} 弱引用数={}",
           leaf.value, leaf.parent.borrow().upgrade(),
           Rc::strong_count(&leaf), Rc::weak_count(&leaf));

          {
            let branch = Rc::new(Node {
              value: 50,
              parent: RefCell::new(Weak::new()),
              children: RefCell::new(vec![]),
            });

            (*branch.children.borrow_mut()).push(Rc::clone(&leaf));
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            // branch,2,0
            println!("leaf: value={} parent={:#?} 强引用数={} 弱引用数={}",
             leaf.value, leaf.parent.borrow().upgrade(),
             Rc::strong_count(&leaf), Rc::weak_count(&leaf));
            // 1,1
            println!("branch: value={} 强引用数={} 弱引用数={}",
             branch.value, Rc::strong_count(&branch), Rc::weak_count(&branch));
          }

          // None,1,0
          println!("leaf: value={} parent={:?} 强引用数={} 弱引用数={}",
           leaf.value, leaf.parent.borrow().upgrade(),
           Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        }
        ```