use std::process::Command;
use std::process::Stdio;

fn rustc_exists() -> bool {
  Command::new("rustc")			               // Run rustc Command
      .args(&["--version"])			          // Commandline Argument
      .stdout(Stdio::null())		          // stdout redirected to null
      .spawn()					                  // spawn subprocess
      .and_then(|mut child| child.wait())	// wait until child process ends
      .map(|status| status.success())	    // Check the return value of child process 
      .unwrap_or(false)
}

fn main() {
  println!("Rustc Command version: {}", rustc_exists());
}
