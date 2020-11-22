use clap::ArgMatches;
use image::{open, DynamicImage};
use lib::cli;

fn main() {
    let matches = cli::build_cli().get_matches();
    
    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();

    match open(input) {
        Ok(image) => {
            let image = edit_photo(image, &matches);
            if let Err(error) = image.save(output) {
                println!("ImageError: {}", error);
            };
        }
        Err(error) => println!("Image Error: {}", error),
    };
}

fn edit_photo(mut image: DynamicImage, matches: &ArgMatches) -> DynamicImage {
    // CROP IMAGE
    if let Some(values) = matches.values_of("crop") {
        let vals: Vec<u32> = values.map(|elem| elem.parse::<u32>().unwrap()).collect();
        image = image.crop_imm(vals[0], vals[1], vals[2], vals[3]);
    }
    // APPLY BLUR
    if let Some(sigma) = matches.value_of("blur") {
        image = image.blur(sigma.parse().unwrap());
    }
    // APPLY CONTRAST IF MATCHES
    if let Some(value) = matches.value_of("contrast") {
        image = image.adjust_contrast(value.parse().unwrap());
    }
    // APPLY BRIGHTEN IF MATCHES
    if let Some(value) = matches.value_of("brighten") {
        image = image.brighten(value.parse().unwrap());
    }
    // APPLY HUE ROTATE IF MATCHES
    if let Some(degrees) = matches.value_of("huerotate") {
        image = image.huerotate(degrees.parse().unwrap());
    }
    // ROTATE IMAGE
    if let Some(degrees) = matches.value_of("rotate") {
        match degrees {
            "90" => image = image.rotate90(),
            "180" => image = image.rotate180(),
            "270" => image = image.rotate270(),
            _ => {}
        }
    }
    // GRAY SCALE
    if matches.is_present("grayscale") {
        image = image.grayscale();
    }
    // INVERT COLORS
    if matches.is_present("invert") {
        image.invert();
    }
    // FLIP VERTICALLY
    if matches.is_present("fliph") {
        image = image.fliph();
    }
    // FLIP HORIZONTALLY
    if matches.is_present("flipv") {
        image = image.flipv();
    }

    image
}
