extern crate clap;
extern crate image;

use lib::cli;

fn main() {
    let matches = cli::build_cli().get_matches();

}

