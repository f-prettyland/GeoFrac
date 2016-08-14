use types::Int as Int;
use types::Float as Float;
pub type Cmplx = (Float, Float);


pub trait FractalGenerator {
    fn get_passes(in_z: (f32, f32), mx: i32, dr: f32) -> i32; 
}

pub struct Mandelbrot {}

impl FractalGenerator for Mandelbrot {
    fn get_passes(in_z: (f32, f32), mx: i32, dr: f32) -> i32 {
        let mut cnt : i32 = 0;
	let c : (f32, f32) = in_z;
	let mut z : (f32, f32) = c;
	loop {
		let xsq : f32 = z.0 * z.0;
		let ysq : f32 = z.1 * z.1;
		//mandelbrot
		z = ((xsq - ysq) + c.0, (2.0 * z.0 * z.1) + c.1);
		cnt += 1;
		if (cnt == mx) || ((xsq + ysq).sqrt() > dr) {
			break;
		}
	}
	cnt
    }
}
