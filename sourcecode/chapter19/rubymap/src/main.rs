use std::collections::HashMap;

macro_rules! rubymap{
    ($($key:expr => $value:expr),*) => {{
        let mut mapping = HashMap::new();
        $(mapping.insert($key, $value); ) *
        mapping
    }};
}

fn main(){
    let students= rubymap!{
        "name" => "Milly",
        "gender"  => "female"  
    };
    println!("students {:?}", students);
}
