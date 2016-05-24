extern crate image;

use std::fs::File;
use std::path::Path;
use std::thread;
use std::sync::mpsc;
use std::f32;

use fracmaths;
use types::*;

#[derive(Clone)]
pub struct Config {
	size: Float,
	step: Float,
	max_iters: Int,
	escape_radius: Float,
	with_color: bool,
	filename: String,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			size: 2.0,
			step: 0.01,
			max_iters: 20,
			escape_radius: 2.0,
			with_color: false,
			filename: "".to_string(),
		}
	}
}

pub struct GifConfig {
	init_frame: Config,
	zoom: Float,
	z_step: Float,
	z_centre_x: Float,
	z_centre_y: Float,
}

impl Default for GifConfig {
	fn default() -> Self {
		GifConfig {
			init_frame: Config::default(),
			zoom: 2.0,
			z_step: 0.1,
			z_centre_x: -0.1011,
			z_centre_y: 0.9563,
		}
	}
}

impl Config {
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

	pub fn escape_radius(mut self, radius: Float) -> Self {
		self.escape_radius = radius;
		self
	}

	pub fn with_color(mut self, col: bool) -> Self {
		self.with_color = col;
		self
	}

	pub fn filename(mut self, filename : Option<String>) -> Self {
		self.filename = match filename {
				Some(f) => f,
				_		=> "".to_string(),
		};
		self
	}

	pub fn gen_loop(&self) {
		let img_dim : Img = ((self.size * 2.0) / self.step) as Img;
		let mut buffer = image::ImageBuffer::new(img_dim, img_dim);

		//Coordinate variable initialisation
		let mut x : Float;
		let mut y : Float = self.size;

		//Channel creation for threads to pass back colours
		let (tx, rx) = mpsc::channel();
		//Greyscale colouring scaler
		let scl : Float = 255.0/(self.max_iters as Float);
		//Values for threads to reduce copying
		let mx = self.max_iters;
		let er = self.escape_radius;
		
		//Y pixel count
		for y_cnt in (0..(img_dim-1)).rev() {
			//x coordinate
			x = -1.0 * self.size;

			//X pixel count
			for x_cnt in 0..(img_dim-1) {
				//Clone sender for channel
				let tx = tx.clone();
				//If not coloured
				if !self.with_color {
					//Creates greyscale thread
					thread::spawn(move || {
						let xy_steps : Int = fracmaths::get_passes_mandelbrot((x, y), mx, er);
						//Calculate colour value for
						tx.send((x_cnt,y_cnt,((scl*(xy_steps as Float)) as u8),None,None)).unwrap();
					});
				} else {
					thread::spawn(move || {
						let xy_steps : Int = fracmaths::get_passes_mandelbrot((x, y), mx, er);
						let mut rgb = (0,0,0);
						if xy_steps != mx {
							rgb = (rgb_val(xy_steps, 0),rgb_val(xy_steps, 1), rgb_val(xy_steps, 2));
						}
						tx.send((x_cnt,y_cnt,rgb.0,Some(rgb.1),Some(rgb.2))).unwrap();
					});
				}
				x += self.step;
			}
			y -= self.step;
		}

		for _ in (0..(img_dim-1)).rev() {
			for _ in 0..(img_dim-1) {
				let (p,q,r,maybeg,maybeb) = rx.recv().unwrap();
				match maybeg {
					Some(g) if maybeb.is_some() =>
							buffer.put_pixel(p, q, image::Rgb([r,g,maybeb.unwrap()])),
					_	=> {
								buffer.put_pixel(p, q, image::Rgb([r,r,r]));
							}
				}
			}
		}

		let b = match self.with_color{
				true => "",
				false=> "-bw",
		};

		let mut filename = &format!("./res/fractal-St{}-Mx{}{}.png", self.step, self.max_iters,b);
		if self.filename != "" {
			filename = &self.filename;
		}
		
		let file = &mut File::create(&Path::new(filename)).unwrap();
		let _ = image::ImageRgb8(buffer).save(file, image::PNG);
	}


}
fn rgb_val(steps : Int, scl : Int) -> u8 {
	((-255.0*
		((steps as Float)*(20.0*(scl as Float)/(f32::consts::PI))).cos()) as Float
		+255.0) as u8
}

impl GifConfig {
	pub fn init_frame(mut self, init: Config) -> Self {
		self.init_frame = init;
		self
	}

	pub fn zoom(mut self, zoom: Float) -> Self {
		self.zoom = zoom;
		self
	}

	pub fn z_step(mut self, step: Float) -> Self {
		self.z_step = step;
		self
	}

	pub fn z_centre_x(mut self, centre: Float) -> Self {
		self.z_centre_x = centre;
		self
	}

	pub fn z_centre_y(mut self, centre: Float) -> Self {
		self.z_centre_y = centre;
		self
	}


	pub fn gif_loop(&self){
		let mut curr_z : Float = 1.0;

		loop{
			let frame = self.init_frame.clone();

			curr_z += self.z_step;
			if curr_z > self.zoom {
				break;
			}
		}

	}
}
