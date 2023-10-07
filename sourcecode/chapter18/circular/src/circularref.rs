use self::List::{Content, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
	Content(i32, RefCell<Rc<List>>),
	Nil,
}

impl List {
	fn tail(&self) -> Option<&RefCell<Rc<List>>> {
		match *self {
			Content(_, ref item) => Some(item),
			Nil => None,
		}
	}
}

pub fn circularloop() {
	let a = Rc::new(Content(100, RefCell::new(Rc::new(Nil))));

	println!("a 最初的引用计数 = {}", Rc::strong_count(&a));
	println!("a 的next指针 = {:?}", a.tail());

	// 检查点1
	let b = Rc::new(Content(200, RefCell::new(Rc::clone(&a))));

	println!("b创建后, a 的引用计数 {}", Rc::strong_count(&a));
	println!("b 最初的引用计数 = {}", Rc::strong_count(&b));
	println!("b的next指针 = {:?}", b.tail());
  
	// 检查点2
	if let Some(link) = a.tail() {
		*link.borrow_mut() = Rc::clone(&b);
	}

	// 检查点3
	println!("a指针变动后，b的引用计数 = {}", Rc::strong_count(&b));
	println!("a指针变动后，a的引用计数 = {}", Rc::strong_count(&a));

	// 如果去掉下面的注释，则会产生循环指针。会导致栈溢出
	//println!("a next item = {:?}", a.tail());
}