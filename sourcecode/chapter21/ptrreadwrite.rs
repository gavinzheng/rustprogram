fn main() { 
  let x = "milly".to_string() ; 
  let y : *const u8 = x . as_ptr() ; 
  unsafe { 
    assert_eq! (y. read () as char ,'m' ) ;
  } 
  
  let x = [0, 10 , 20 , 30]; 
  let y = x[0.. ].as_ptr() as *const [u32 ; 4]; 
  unsafe { 
    assert_eq!(y.read(), [0 , 10 , 20 , 30]);
  } 
  
  let x = vec! [0 , 10 , 20 , 30]; 
  let y = &x as *const Vec<i32> ;
  unsafe { 
    assert_eq!(y.read() , [0 , 10 , 20 , 30]); 
  }
  
  let mut x = ""; 
  let y = &mut x as *mut &str ; 
  let z = "hust"; 
  unsafe { 
    y.write(z) ;
    assert_eq!(y.read () ,"hust" ); 
  }
}