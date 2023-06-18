use std::collections::HashMap; 

fn main(){
let mut majors_univ = HashMap::new(); 
majors_univ.insert("uofT", vec!["Rotman", "Mechanics"]);
majors_univ.insert("uyoark", vec!["Business", "Math"]); 
majors_univ.insert("hust", vec!["Computer Science", "Physics"]); 
let universities = ["uofT", "uyoark", "hust"]; 
for &univ in universities.iter().flat_map(|x| 
&majors_univ[x]) { 
println!("{}", univ ); 
} 
}