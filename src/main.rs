mod chip8;
mod isa;
mod tui;

use std::env;
use std::fs;
use std::io;
use std::process;
use tui::App;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <rom_file>", args[0]);
        process::exit(1);
    }

    let rom = match fs::read(&args[1]) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read ROM file '{}': {}", args[1], e);
            process::exit(1);
        }
    };

    if rom.len() > 3584 {
        eprintln!("ROM file is too large ({} bytes). Maximum size is 3584 bytes.", rom.len());
        process::exit(1);
    }

    let mut terminal = ratatui::init();
    let app_result = App::new(&rom).run(&mut terminal);
    ratatui::restore();
    app_result
}
