use image::{DynamicImage, open};
use lib::{cli};

fn main() {
    let matches = cli::build_cli().get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();
    let image = open(input).unwrap();

    image.save(output).unwrap();
}

