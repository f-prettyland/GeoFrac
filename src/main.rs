extern crate getopts;

mod terminal;
mod types;
mod fracmaths;
mod graphic;

use types::*;

use std::env;

fn main() {
	//Bound size, Step size, Max passes, Divergence radius
	let args: Vec<_> = env::args().collect();
	let program = args[0].clone();
	
	let mut opts = getopts::Options::new();

	opts.optopt("f", "filename", "Set the resultant filename", "FILENAME");
	opts.optflag("g", "gif", "Generate a gif instead with three more parameters: ZOOM, ZOOM_STEP, Z_CENTRE_X, Z_CENTRE_Y");
	opts.optflag("h", "help", "Display this help message");
	opts.optflag("b", "no-color", "Black and white output");
	opts.optflag("t", "term", "Terminal output (ignores all other args)");

	let matches = match opts.parse(&args[1..]) {
		Ok(m) => m,
		Err(f) => panic!(f.to_string()),
	};

	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}

	if matches.opt_present("t") {
		terminal::gen_term_loop();
		return;
	}

	let col = !matches.opt_present("b");

	let filename = matches.opt_str("f");
	let gif_flag = matches.opt_present("g");

	let settings = matches.free;
	let settings_len = settings.len();

	if settings_len == 8 && gif_flag{ 
		if let Ok(configs) = parse_settings(&settings, col, filename) {
			if let Ok(mut gif_configs) = parse_gif_settings(&settings, configs) {
				gif_configs.gif_loop();
				return;
			}
		}
	}else if settings_len == 4 {
		if let Ok(configs) = parse_settings(&settings, col, filename) {
			configs.gen_loop();
			return;
		}
	}
	
	print_usage(&program, opts);
}

// Should be a nicer way of doing this
// First attempt at a clean up
macro_rules! parse_setting {
	($setting:ident, $setting_type:ident, $position:expr, $settings:ident) => (
		let $setting: $setting_type = match $settings[$position].parse() {
			Ok(value) => value,
			Err(_) => return Err(SettingsError::from_position($position)),
		}
	);
}

fn parse_settings(settings: &Vec<String>, col : bool, filename : Option<String>) -> Result<graphic::Config, SettingsError> {
	parse_setting!(size, Float, 0, settings);
	parse_setting!(step, Float, 1, settings);
	parse_setting!(iters, Int, 2, settings);
	parse_setting!(radius, Float, 3, settings);

	Ok(graphic::Config::default()
		.size(size)
		.step(step)
		.max_iters(iters)
		.escape_radius(radius)
		.with_color(col)
		.filename(filename)
		)
 }


fn parse_gif_settings(settings: &Vec<String>, con : graphic::Config) -> Result<graphic::GifConfig, SettingsError> {
	parse_setting!(zoom, Float, 4, settings);
	parse_setting!(z_step, Float, 5, settings);
	parse_setting!(z_x, Float, 6, settings);
	parse_setting!(z_y, Float, 7, settings);

	Ok(graphic::GifConfig::default()
		.init_frame(con)
		.zoom(zoom)
		.z_step(z_step)
		.z_centre_x((-1.0)*z_x)
		.z_centre_y(z_y))
}

enum SettingsError {
	Size,
	Step,
	Iters,
	Radius,
	Unknown,
}

impl SettingsError {
	fn from_position(position: usize) -> Self {
		match position {
			0 => SettingsError::Size,
			1 => SettingsError::Step,
			2 => SettingsError::Iters,
			3 => SettingsError::Radius,
			_ => SettingsError::Unknown,
		} 
	}
}

fn print_usage(program: &str, opts: getopts::Options) {
	let brief = format!("Usage: {} SIZE STEP MAX_ITERATIONS ESCAPE_RADIUS [options]", program);
	print!("{}", opts.usage(&brief));
}

