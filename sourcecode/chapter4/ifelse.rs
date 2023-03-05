// from Chapter 3/code/loops.rs
fn main() {
	let adult = true;
	let age = if adult { "+18" } else { "-18" };
	println!("Age is {}", age); // Age is +18

	//let health = 48;
	//let result = if health <=0 { "Game over man!" };
	let mut power = 1;
	
	loop {
		power += 1;
		if power == 42 {
			// Skip the rest of this iteration
			continue;
		}
		print!("{} ", power);
		if power == 50 {
			print!("OK, that's enough for today");
			break; // exit the loop
		}
	}

}