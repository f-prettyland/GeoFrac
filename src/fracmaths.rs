use types::Int as Int;
use types::Float as Float;
pub type Cmplx = (Float, Float);

pub fn get_passes_mandelbrot(in_z: Cmplx, mx: Int, dr: Float) -> Int {
	let mut cnt : Int = 0;
	let c : Cmplx = in_z;
	let mut z : Cmplx = c;
	loop {
		let xsq : Float = z.0*z.0;
		let ysq : Float = z.1*z.1;
		//mandelbrot
		z = ((xsq-ysq)+c.0, (2.0*z.0*z.1)+c.1);
		cnt += 1;
		if (cnt == mx) || ((xsq+ysq).sqrt()>dr) {
			break;
		}
	}
	cnt
}