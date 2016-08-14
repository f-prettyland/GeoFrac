#[macro_use]
extern crate clap;

mod terminal;
mod types;
mod fracmaths;
mod graphic;
mod renderer;

use clap::{App, Arg, SubCommand};

use std::io;
use std::fs;
use renderer::FromMatches;

fn main() {

    let options = [
	Arg::with_name("x")
	    .help("X coordinate for image center point.")
	    .required(true),
	Arg::with_name("y")
	    .help("Y coordinate for image center point.")
	    .required(true),
	Arg::with_name("size")
	    .help("Side length of area to be rendered.")
	    .required(true),
	Arg::with_name("step")
	    .help("TODO: step help")
	    .required(true),
	Arg::with_name("iterations")
	    .help("TODO: iterations help")
	    .required(true),
	Arg::with_name("radius")
	    .help("TODO: radius help")
	    .takes_value(true)
	    .short("r")
	    .long("radius"),
	Arg::with_name("greyscale")
	    .help("Disable color rendering.")
	    .short("g")
	    .long("grey"),
	Arg::with_name("output")
	    .help("Specify path to save output file.")
	    .takes_value(true)
	    .short("o")
	    .long("output")
    ];

    let app = App::new("GeoFrac")
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
			.required(true),
		    Arg::with_name("zstep")
			.help("TODO: zstep help")
			.required(true)	
		]),
	    SubCommand::with_name("term")
		.about("Outputs to terminal")
		.help("TODO: terminal help")
	]);


    // TODO: Attempt to remove cloning
    
    let clone = app.clone();
    let matches = app.get_matches();
    
    match matches.subcommand_name() {
	Some("term") => { terminal::gen_term_loop() },
	Some("still") => {
	    let options = &matches.subcommand_matches("still").unwrap();
	    render_still(options);
	},
	Some("gif") => {
	    let options = &matches.subcommand_matches("gif").unwrap();
	    if let Err(_) = render_gif(options) {
		println!("Could not create output directory: \"/gif\"");
	    };
	},
	_ => { clone.print_help().unwrap(); }
    }
}

fn render_still(matches: &clap::ArgMatches) {
    renderer::Renderer::from_matches(matches).render::<fracmaths::Mandelbrot>()
}

fn render_gif(matches: &clap::ArgMatches) -> io::Result<()> {
    try!(fs::create_dir_all(renderer::GIF_OUT_DIR));
    renderer::GifRenderer::from_matches(matches).render::<fracmaths::Mandelbrot>();
    Ok(())
}
