enum Day { 
  Monday, 
  Tuesday, 
  Wednesday, 
  Thursday, 
  Friday, 
  Saturday, 
  Sunday, 
  } 
  impl Day { 
  fn mood(&self) { 		// 方法
  println!("{}", match *self { 
  Day::Monday => "走向深渊!", 
  Day::Tuesday => "路漫漫!", 
  Day::Wednesday => "夜茫茫!", 
  Day::Thursday => "黎明在前方!", 
  Day::Friday => "胜利大逃亡!", 
  _ => "周末...", 
  }) 
  } 
  } 
  fn main() { 
  let today = Day::Tuesday; 
  today.mood(); 
  }