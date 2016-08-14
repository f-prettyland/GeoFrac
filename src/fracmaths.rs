use num::complex::Complex;

pub trait FractalGenerator {
    fn get_passes(in_z: (f32, f32), mx: i32, dr: f32) -> i32; 
}

pub struct Mandelbrot {}

impl FractalGenerator for Mandelbrot {
    fn get_passes(in_z: (f32, f32), mx: i32, dr: f32) -> i32 {
        let c = Complex::new(in_z.0, in_z.1);
        let mut cnt = 0;
	let mut z  = c;
        for _ in 0..mx {
	    z = z * z + c;
	    cnt += 1;
	    if cnt == mx || z.norm() > dr { break }
	}
	cnt
    }
}
