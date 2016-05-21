extern crate image;

use std::fs::File;
use std::path::Path;
use std::thread;
use std::sync::mpsc;

use fracmaths;
use types::Int as Int;
use types::Float as Float;
use types::Img as Img;

pub fn gen_loop(size : Float, stp : Float, mx : Int, dr : Float, bw : bool){
	let img_dim : Img = ((size*2.0)/stp) as Img;
	let mut buffer = image::ImageBuffer::new(img_dim, img_dim);

	let scl : Float = 255.0/(mx as Float);
	let mut x : Float;
	let mut y : Float = size;

	let (tx, rx) = mpsc::channel();
	
	for y_cnt in (0..(img_dim-1)).rev() {
		x = -1.0*size;
		for x_cnt in 0..(img_dim-1) {
			let tx = tx.clone();
			thread::spawn(move || {
						let xy_steps : Int = fracmaths::get_passes_mandelbrot((x, y), mx, dr);
						let mut r : u8;
						let mut g : u8;
						let mut b : u8;
						if xy_steps == mx {
							//Black if max
							(r,g,b) = (0,0,0);
						}else{
							(r,g,b) = (rgb_val(xy_steps, 0),rgb_val(xy_steps, 1), rgb_val(xy_steps, 2));
						}

						tx.send((x_cnt,y_cnt,r,g,b)).unwrap();
					});
			x += stp;
		}
		y -= stp;
	}

	for _ in (0..(img_dim-1)).rev() {
		for _ in 0..(img_dim-1) {
			let (p,q,r,g,b) = rx.recv().unwrap();
			buffer.put_pixel(p, q, image::Rgb([r,g,b]));
			// buffer.put_pixel(p, q, image::Luma([(scl*(z as Float)) as u8]));
		}
	}

	let ref mut path = File::create(&Path::new("./res/fractal.png")).unwrap();
	// let _ = image::ImageLuma8(buffer).save(path, image::PNG);
	let _ = image::ImageRgb8(buffer).save(path, image::PNG);
}

fn rgb_val(steps : Int, scl : Int) -> u8 {
	((-255.0*
		((steps as Float)*(20.0*(scl as Float)/(Float::consts::PI))).cos()) as Float
		+255.0) as u8
}