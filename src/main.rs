#[macro_use]
extern crate clap;

mod terminal;
mod types;
mod fracmaths;
mod graphic;

use clap::{App, Arg, SubCommand};

fn main() {
    let options = [
	Arg::with_name("size")
	    .help("TODO: size help")
	    .index(1)
	    .required(true),
	Arg::with_name("step")
	    .help("TODO: step help")
	    .index(2)
	    .required(true),
	Arg::with_name("iterations")
	    .help("TODO: iterations help")
	    .index(3)
	    .required(true),
	Arg::with_name("radius")
	    .help("TODO: radius help")
            .takes_value(true)
	    .short("r")
            .long("radius"),
	Arg::with_name("greyscale")
	    .help("TODO: greyscale help")
	    .short("g")
	    .long("grey"),
	Arg::with_name("output")
	    .help("TODO: output help")
	    .takes_value(true)
	    .short("o")
	    .long("output")
    ];

    let matches = App::new("GeoFrac")
	.subcommands(vec![
	    SubCommand::with_name("still")
		.about("Generates a still image")
		.args(&options),
	    SubCommand::with_name("gif")
		.about("Generates an animated gif")
		.args(&options)
		.args(&[
		    Arg::with_name("zoom")
			.help("TODO: zoom help")
			.index(5)
			.required(true),
		    Arg::with_name("zstep")
			.help("TODO: zstep help")
			.index(6)
			.required(true),
		    Arg::with_name("x")
			.help("TODO: x help")
			.index(7)
			.required(true),
		    Arg::with_name("y")
			.help("TODO: y help")
			.index(8)
			.required(true),
		]),
	    SubCommand::with_name("term")
		.about("Outputs to terminal")
		.help("TODO: terminal help")
	]).get_matches();



    match matches.subcommand_name() {
	Some("still") => run_still(&matches.subcommand_matches("still").unwrap()),
	Some("gif") => run_gif(&matches.subcommand_matches("gif").unwrap()),
	Some("term") => terminal::gen_term_loop(),
	None => println!("None"),
	_ => println!("Not recognised"),
    }
}

fn run_still(matches: &clap::ArgMatches) {
    frame_setup(matches).run()
}

fn frame_setup(matches: &clap::ArgMatches) -> graphic::Config {
    let size = value_t!(matches, "size", f32).unwrap_or_else(|e| e.exit());
    let step = value_t!(matches, "step", f32).unwrap_or_else(|e| e.exit());
    let iter = value_t!(matches, "iterations", i32).unwrap_or_else(|e| e.exit());
//    let radius = value_t!(matches, "radius", f32).unwrap_or_else(|e| e.exit());

    let mut generator = graphic::Config::new(size, step, iter);

    if matches.is_present("greyscale") {
	generator = generator.greyscale();
    }

    if matches.is_present("output") {
	generator = generator.filename(matches.value_of("output").unwrap().to_string());
    } 

    if matches.is_present("radius") {
        generator = generator.escape_radius(matches.value_of("radius").unwrap().parse().expect("Could not parse escape radius value"))
    }

    generator
}

fn run_gif(matches: &clap::ArgMatches) {
    let zoom = value_t!(matches, "zoom", f32).unwrap_or_else(|e| e.exit());
    let zstep = value_t!(matches, "zstep", f32).unwrap_or_else(|e| e.exit());
    let x = value_t!(matches, "x", f32).unwrap_or_else(|e| e.exit());
    let y = value_t!(matches, "y", f32).unwrap_or_else(|e| e.exit());
    
    let frame_generator = frame_setup(matches);

    graphic::GifConfig::new(frame_generator, zoom, zstep, x, y).run();
}

