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

            let config = graphic::Config::default()
                .size(parse_string(&args[1]))
                .step(parse_string(&args[2]))
                .max_iters(parse_string(&args[3]))
                .escape_radius(parse_string(&args[4]))
                .with_color();

            // if !args.contains(&"-t".to_string()) {
            //     let config = config.with_color();
            // }

            config.gen_loop();
	}

	if args.contains(&"-t".to_string()) {
		terminal::gen_term_loop()
	}
}

fn print_help(){
	println!("First four arguments should be");
	println!("	1. Bound size");
	println!("	2. Step size");
	println!("	3. Max iterations");
	println!("	4. Divergence radius");
	println!("");
	println!("Flags ");
	println!("	-t	Output terminal [ignores all other arguments]");
	println!("	-h	Display this help message");
	println!("	-b	Black and white");
}

/// # Generic read in
///	## Args
///	1. String to parse
///	## Output
///	1. Generic type (which implements `FromStr` trait)
fn parse_string<T: std::str::FromStr>(to_parse: &String) -> T {
	to_parse.trim().parse().unwrap_or_else(|_|{
		println!("Incorrect arguments format");
		print_help();
		process::exit(1);
		})
}
