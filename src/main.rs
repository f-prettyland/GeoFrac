extern crate getopts;

mod terminal;
mod types;
mod fracmaths;
mod graphic;

use std::env;



fn main() {
    //Bound size, Step size, Max passes, Divergence radius
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();
    
    let mut opts = getopts::Options::new();

    opts.optflag("h", "help", "Display this help message");
    opts.optflag("b", "no-color", "Black and white output");
    opts.optflag("t", "term", "Terminal output (ignores all other args)");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("t") {
        terminal::gen_term_loop();
        return;
    }

    let settings = matches.free;

    if settings.len() == 4 {
        if let Ok(configs) = parse_settings(settings) {
            configs.gen_loop();
            return;
        }
    }
    print_usage(&program, opts);
}

// Must be a nicer way of handling this
fn parse_settings(settings: Vec<String>) -> Result<graphic::Config, SettingsError> {
    if let Ok(size) = settings[0].parse() {
        if let Ok(step) = settings[1].parse() {
            if let Ok(iters) = settings[2].parse() {
                if let Ok(radius) = settings[3].parse() {
                    Ok(graphic::Config::default()          
                       .size(size)
                       .step(step)
                       .max_iters(iters)
                       .escape_radius(radius)
                       .with_color())
                } else { return Err(SettingsError::Radius) } 
            } else { return Err(SettingsError::Iters) } 
        } else { return Err(SettingsError::Step) } 
    } else { return Err(SettingsError::Size) }
}

enum SettingsError {
    Size,
    Step,
    Iters,
    Radius,
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} SIZE STEP MAX_ITERATIONS ESCAPE_RADIUS [options]", program);
    print!("{}", opts.usage(&brief));
}

