extern crate image;

use std::fs::File;
use std::path::Path;
use std::thread;
use std::sync::mpsc;

use fracmaths;
use types::Int as Int;
use types::Float as Float;
use types::Img as Img;
use std::f32;

#[derive(Clone)]
pub struct Config {
	size: Float,
	step: Float,
	max_iters: Int,
	escape_radius: Float,
	with_color: bool,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			size: 2.0,
			step: 0.01,
			max_iters: 20,
			escape_radius: 2.0,
			with_color: false,
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

	pub fn with_color(mut self) -> Self {
		self.with_color = true;
		self
	}


	pub fn gen_loop(&self) {
	let img_dim : Img = ((self.size * 2.0) / self.step) as Img;
	let mut buffer = image::ImageBuffer::new(img_dim, img_dim);

	// 
	let mut x : Float;
	let mut y : Float = self.size;

	let (tx, rx) = mpsc::channel();
	let scl : Float = 255.0/(mx as Float);
	
	for y_cnt in (0..(img_dim-1)).rev() {
		x = -1.0 * self.size;
		for x_cnt in 0..(img_dim-1) {
		let tx = tx.clone();

				let conf = self.clone();
		thread::spawn(move || {
			let xy_steps : Int = fracmaths::get_passes_mandelbrot((x, y),
																		  conf.max_iters,
																		  conf.escape_radius);

			let mut rgb = (0,0,0);
			if xy_steps != conf.max_iters {
			rgb = (rgb_val(xy_steps, 0),rgb_val(xy_steps, 1), rgb_val(xy_steps, 2));
			}
			tx.send((x_cnt,y_cnt,rgb.0,rgb.1,rgb.2)).unwrap();
		});
		x += self.step;
		}
		y -= self.step;
	}

	for _ in (0..(img_dim-1)).rev() {
		for _ in 0..(img_dim-1) {
		let (p,q,r,g,b) = rx.recv().unwrap();
		buffer.put_pixel(p, q, image::Rgb([r,g,b]));
		// buffer.put_pixel(p, q, image::Luma([(scl*(z as Float)) as u8]));
		}
	}

	let file = &mut File::create(&Path::new(&format!("./res/fractal-St{}-Mx{}.png", self.step, self.max_iters))).unwrap();
	// let _ = image::ImageLuma8(buffer).save(path, image::PNG);
	let _ = image::ImageRgb8(buffer).save(file, image::PNG);
	}
}

fn rgb_val(steps : Int, scl : Int) -> u8 {
	((-255.0*
		((steps as Float)*(20.0*(scl as Float)/(f32::consts::PI))).cos()) as Float
		+255.0) as u8
}

// pub fn gif_loop(size : Float, stp : Float, mx : Int, dr : Float, bw : bool, zoom : Int){

// }