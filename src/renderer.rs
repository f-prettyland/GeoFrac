use clap;
use types::*;

pub const GIF_OUT_DIR: &'static str = "gif";

#[derive(Debug, Clone)]
pub struct Renderer {
	pub size: Float,
	pub step: Float,
	pub max_iters: Int,
	pub escape_radius: Float,
	pub with_color: bool,
	pub filename: String,
	pub begin_x: Float,
	pub begin_y: Float,
}

pub trait FromMatches {
	fn from_matches(matches: &clap::ArgMatches) -> Self;
}

impl Renderer {
	pub fn size(mut self, size: Float) -> Self {
		self.size = size;
		self
	}

	pub fn step(mut self, step: Float) -> Self {
		self.step = step;
		self
	}

	pub fn max_iters(mut self, iters: Int) -> Self {
		self.max_iters = iters;
		self
	}

	pub fn filename(mut self, filename : String) -> Self {
	    self.filename = filename;
	    self
	}

	pub fn begin_x(mut self, x : Float) -> Self {
	    self.begin_x = x;
	    self
	}

	pub fn begin_y(mut self, y : Float) -> Self {
	    self.begin_y = y;
	    self
	}
}

impl FromMatches for Renderer {
	fn from_matches(matches: &clap::ArgMatches) -> Self {
		let x = value_t!(matches, "x", f32).unwrap();
		let y = value_t!(matches, "y", f32).unwrap();
		let size = value_t!(matches, "size", f32).unwrap();
		let step = value_t!(matches, "step", f32).unwrap();
		let iter = value_t!(matches, "iterations", i32).unwrap();
		let grey = matches.is_present("greyscale");

		let filename = if matches.is_present("output") {
			matches.value_of("output").unwrap().to_string()
		} else{
			let grey_flag = match grey{
				true => "",
				false=> "-bw",
			};
			format!("./res/fractal-St{}-Mx{}{}.png", step, iter, grey_flag)
		};

		let radius = if matches.is_present("radius") {
			matches.value_of("radius").unwrap().parse().expect("Could not parse escape radius value")
		}else{
			2.0
		};

		Renderer {
			size: 0.5 * size,
			step: step,
			max_iters: iter,
			escape_radius: radius,
			with_color: !grey,
			filename: filename,
			begin_x: x - 0.5 * size,
			begin_y: y + 0.5 * size,
		} 
	}
}

#[derive(Debug)]
pub struct GifRenderer {
	pub init_frame: Renderer,
	pub zoom: Float,
	pub z_step: Float,
	pub z_centre_x: Float,
	pub z_centre_y: Float,
	pub output_dir: String,
}

impl FromMatches for GifRenderer {
	fn from_matches(matches: &clap::ArgMatches) -> Self {
		let zoom = value_t!(matches, "zoom", f32).unwrap();
		let z_step = value_t!(matches, "zstep", f32).unwrap();
		let x = value_t!(matches, "x", f32).unwrap();
		let y = value_t!(matches, "y", f32).unwrap();

		GifRenderer {
			init_frame: Renderer::from_matches(matches),
			zoom: zoom,
			z_step: z_step,
			z_centre_x: x,
			z_centre_y: y,
			output_dir: GIF_OUT_DIR.to_string(),
		}
	}
}
