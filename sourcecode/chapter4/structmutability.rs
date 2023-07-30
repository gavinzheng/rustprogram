struct S1 { 
	field1: i32, 
	field2: S2 
}
struct S2 { 
	field: i32 
}
fn main() { 
	let s = S1 { field1: 45, field2: S2 { field: 23 } }; 
	// s 是深度不可变的，所以下面的可变赋值是会出错的
	// s.field1 = 46; 
	// s.field2.field = 24; 
	let mut s = S1 { field1: 45, field2: S2 { field: 23 } }; 
	// s本身是可变的，所以对其域成员的复制是OK的 
	s.field1 = 46; 
	s.field2.field = 24; 
} 