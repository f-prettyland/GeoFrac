extern crate colored;
// https://github.com/mackwic/colored
// black
// red
// green
// yellow
// blue
// magenta (or purple)
// cyan
// white

use colored::*;
use std::io;

type Int = i32;
type Float = f64;
type Cmplx = (Float, Float);

fn main() {
	println!("Mandelbrot'");
	let size : Float = get_in("Bound size");
	let stp : Float = get_in("Step size");
	let mx : Int = get_in("Max passes");
	let dr : Float = get_in("Diveregence radius");
	// let size : Float = 2.0;
	// let stp : Float = 0.04;
	// let mx : Int = 7;
	// let dr : Float = 2.0;
	
	let mut x : Float;
	let mut y : Float = size;
	loop{
		x = -1.0*size;
		loop{
			let xy_steps : Int = get_passes_mandelbrot((x, y), mx, dr);

			// print!("({0:.2},{1:.2},p{2}) ",x,y, xy_steps);
			print!("{}", get_col_out(xy_steps));

			x += stp;
			if x > size {
				break;
			}

		}

		println!("");
		y -= stp;
		if y < -1.0*size {
			break;
		}

	}
}

/// #Generic read in
///	##Args
///	1. String to show user before asking for input
///	##Output
///	1. Generic type (which implements FromStr trait)
fn get_in<T: std::str::FromStr>(said: &str) -> T {
	loop {
		println!("{}", said);

		let mut instr = String::new();

		io::stdin().read_line(&mut instr)
			.expect("failed to read line");

		return match instr.trim().parse() {
					Ok(num) => num,
					Err(_) =>{
						println!("Incorrect format");
						continue;
					} 
				};

	}
}

fn get_col_out(steps : Int) -> ColoredString{
	match steps {
		1 => "█".red(),
		2 => "█".green(),
		3 => "█".yellow(),
		4 => "█".blue(),
		5 => "█".magenta(),
		6 => "█".cyan(),
		_ => "█".white(),
	}
}

fn get_passes_mandelbrot(in_z: Cmplx, mx: Int, dr: Float) -> Int {
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

//TODO Genericafy
// fn get_passes(in_z: Cmplx, mx: Int, dr: Float) -> Int {
// 	let mut cnt : Int = 0;
// 	let c : Cmplx = in_z;
// 	let mut z : Cmplx = c;
// 	loop {
// 		z = mandelbrot(z,c);
// 		cnt += 1;
// 		if (cnt == mx) || (z.0>dr) || (z.1>dr) {
// 			break;
// 		}
// 	}
// 	cnt
// }