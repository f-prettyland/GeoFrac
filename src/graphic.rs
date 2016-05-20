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

	let mut x : Float;
	let mut x_cnt : Img = 0;
	let mut y : Float = size;
	let mut y_cnt : Img = img_dim;
	loop{
		x = -1.0*size;
		loop{
			let xy_steps : Int = fracmaths::get_passes_mandelbrot((x, y), mx, dr);

			buffer.put_pixel(x_cnt, y_cnt, image::Luma([xy_steps as u8]));

			x += stp;
			x_cnt += 1;
			if x_cnt > img_dim {
				break;
			}
		}

		if y_cnt == 0{
			break;
		}
		y -= stp;
		y_cnt -= 1;
	}


	let ref mut path = File::create(&Path::new("fractal.png")).unwrap();
	let _    = image::ImageLuma8(buffer).save(path, image::PNG);
}
