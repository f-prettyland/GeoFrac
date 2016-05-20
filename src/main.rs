mod terminal;
mod types;
mod fracmaths;
mod graphic;
use std::env;
use std::process;

fn main() {
	//Bound size, Step size, Max passes, Divergence radius
	let args: Vec<_> = env::args().collect();
	
	if args.contains(&"-h".to_string()) || args.len() == 1 {
		print_help();
		return;
	}

	if args.len() >= 5 {
		graphic::gen_loop(parse_string(&args[1]), parse_string(&args[2]),
			parse_string(&args[3]), parse_string(&args[4]))
	}

	if args.contains(&"-t".to_string()) {
		terminal::gen_term_loop();
		return;
	}
}

fn print_help(){
	println!("First four arguments should be");
	println!("	1. Bound size");
	println!("	2. Step size");
	println!("	3. Max iterations");
	println!("	4. Divergence radius");
	println!("");
	println!("Unless ");
	println!("	-t flag		Output terminal");
}

/// #Generic read in
///	##Args
///	1. String to parse
///	##Output
///	1. Generic type (which implements FromStr trait)
fn parse_string<T: std::str::FromStr>(to_parse: &String) -> T {
	return match to_parse.trim().parse() {
				Ok(num) => num,
				Err(_) =>{
					println!("Incorrect arguments format");
					print_help();
            		process::exit(1);
				} 
			};
}