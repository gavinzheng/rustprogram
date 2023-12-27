fn main(){
  let a = [10, 20, 30, 40, 50, 60]; 	// 数组类型
  let complete = &a[..]; 			// 包含数组a的所有元素的切片

  // 包含数组a索引位置1到4(不包含4)的元素的切片，a[1],a[2],a[3],不包含a[4], var1内容为[20,30,40] 
  let var1 = &a[1..4];   
  let var2 = &a[0..=2]; 	// 包含a数组的头3个元素a[0]，a[1],a[2],var2内容为[10,20,30]
  let var3 = &a[2..];  	// 包含第2个元素a[2]之后(包含2)所有的元素，var3内容为[30，40，50，60] 
  println!("{:?} \n{:?} \n{:?}",var1,var2,var3);
}