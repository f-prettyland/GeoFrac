extern crate colored;
use self::colored::*;

use fracmaths;
use fracmaths::FractalGenerator;
use types::Int as Int;
use types::Float as Float;

pub fn gen_term_loop(){
	let size : Float = 2.0;
	let stp : Float = 0.04;
	let mx : Int = 8;
	let dr : Float = 2.0;
	
	let mut x : Float;
	let mut y : Float = size;
	loop{
		x = -1.0*size;
		loop{
			print!("{}", get_col_out(
					fracmaths::Mandelbrot::get_passes((x, y), mx, dr))
					);
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

/// #Coloured block output
///	##Args
///	1. The number of steps taken
///	##Output
///	1. ASCII block char coloured according to number of steps taken
fn get_col_out(steps : Int) -> ColoredString{
	match steps {
		1 => "█".red(),
		2 => "█".green(),
		3 => "█".yellow(),
		4 => "█".blue(),
		5 => "█".magenta(),
		6 => "█".cyan(),
		7 => "█".black(),
		_ => "█".white(),
	}
}
