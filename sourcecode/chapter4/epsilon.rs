fn main() { 
    let result: f32 = 0.1 + 0.1; 
    let desired: f32 = 0.2; 
    let absolute_difference = (desired - result).abs(); 	// 计算两个浮点数之间的差别
    assert!(absolute_difference <= f32::EPSILON); 		// 差别在预定义的EPSILON范围内,就认为是相等
  }