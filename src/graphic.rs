extern crate image;

use std::fs::File;
use std::path::Path;
use std::thread;
use std::sync::mpsc;
use std::f32;

use fracmaths;
use types::*;
use config;

impl config::Config {
	pub fn run(&self) {
	let img_dim : Img = (self.size*2.0 / self.step) as Img;
	let mut buffer = image::ImageBuffer::new(img_dim, img_dim);

	//Coordinate variable initialisation
	let mut x : Float;
	let mut y : Float = self.begin_y;

	//Channel creation for threads to pass back colours
	let (tx, rx) = mpsc::channel();
	//Greyscale colouring scaler
	let scl : Float = 255.0/(self.max_iters as Float);
	//Values for threads to reduce copying
	let mx = self.max_iters;
	let er = self.escape_radius;
	
	//Y pixel count
	for y_cnt in 0..(img_dim-1) {
		//x coordinate
		x = self.begin_x;

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

	let file = &mut File::create(&Path::new(&self.filename)).unwrap();
	let _ = image::ImageRgb8(buffer).save(file, image::PNG);
	}


}

fn rgb_val(steps : Int, scl : Int) -> u8 {
	((-255.0*
	  ((steps as Float)*(20.0*(scl as Float)/(f32::consts::PI))).cos()) as Float
	 +255.0) as u8
}


impl config::GifConfig {

	pub fn run(&self) {
		println!("generating...");

		let init_size = self.init_frame.size;
		
		let mut curr_z = 1.0;
		let mut count = 0;
		let mut frame = self.init_frame.clone();
		loop {
			count += 1;

			curr_z += self.z_step;
			if curr_z > self.zoom { break; }
			
			println!("Iteration: {}", count);
				
			let begin_x =
				if init_size < self.z_centre_x + frame.size {
					init_size - 2.0 * frame.size
				} else {
					f32::max(self.z_centre_x - frame.size, -init_size)
				};
			
			let begin_y =
				if -init_size > self.z_centre_y - frame.size {
				   -init_size + 2.0 * frame.size 
				} else {
					f32::min(self.z_centre_y + frame.size, init_size)
				};
				
			frame = frame.clone()
				.size((1.0 / curr_z) * frame.size)
				.step((1.0 / curr_z) * frame.step)
				.max_iters((curr_z * frame.max_iters as Float) as Int)
				.filename(format!("./{}/{}.png", self.output_dir, count))
				.begin_x(begin_x)
				.begin_y(begin_y);

			frame.run();
		}
	}
}