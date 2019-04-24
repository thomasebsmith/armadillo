use std::process;

fn main() {
    if let Err(error) = armadillo::run() {
        println!("{}", error);
        process::exit(1);
    }
}
