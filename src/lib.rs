use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
	println!("Hello from Armadillo!");
	Ok(())
}
