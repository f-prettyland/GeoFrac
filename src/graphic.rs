extern crate image;

use std::fs::File;
use std::path::Path;

use fracmaths;
use types::Int as Int;
use types::Float as Float;
use types::Img as Img;

pub fn gen_loop(size : Float, stp : Float, mx : Int, dr : Float){

	let img_dim : Img = ((size*2.0)/stp) as Img;
    let mut buffer = image::ImageBuffer::new(img_dim, img_dim);

    let scl : Float = 255.0/(mx as Float);

	let mut x : Float;
	let mut y : Float = size;
	
	for y_cnt in (0..(img_dim-1)).rev() {
		x = -1.0*size;
		for x_cnt in 0..(img_dim-1) {
			let xy_steps : Int = fracmaths::get_passes_mandelbrot((x, y), mx, dr);

			buffer.put_pixel(x_cnt, y_cnt, image::Luma([(scl*(xy_steps as Float)) as u8]));

			x += stp;
		}
		y -= stp;
	}


	let ref mut path = File::create(&Path::new("./res/fractal.png")).unwrap();
	let _    = image::ImageLuma8(buffer).save(path, image::PNG);
}
