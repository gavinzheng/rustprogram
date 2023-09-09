fn main () { 
	assert_eq!((l.. 5) , std::ops::Range{ start: 1 , end : 5 }); //左闭右开
	assert_eq! ((1.. =5) , std::ops:Rangeinclusive: new(l , 5) );//全闭
	assert_eq! (3+4+5 , (3 .. 6).sum()); 
	assert_eq! (3+4+5+6, (3 .. =6).sum( )); 

	//左闭右开
	for l in (1..5) { 
		Println! （“{}”,l) //  1 , 2 , 3 , 4 
	}
	//全闭
	for i in (1..=5 ) { 
		Println! （“{}”, i); // 1 , 2 , 3 ,4,5  
    }
}