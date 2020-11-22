use clap::{App, Arg, ArgGroup};

/// This function builds the Command Line Application
/// already with its metadata, args boundaries etc.
pub fn build_cli() -> App<'static, 'static> {
    App::new("Image Editor application")
        // METADATA
        .author(crate_authors!())
        .version(crate_version!())
        .about("Edit, transform, crop, rotate etc. an image with just one command")
        .after_help("Have a nice day!")
        // POSITIONAL ARGUMENTS
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input image file to edit")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output path to store the image (defaults to the input + a number)")
                .required(true)
                .index(2),
        )
        // OPTIONALS GROUP, AT LEAST ONE OPTION IS REQUIRED
        .group(
            ArgGroup::with_name("options")
            .required(true)
            .multiple(true)
        )
        .group(
            ArgGroup::with_name("colors")
            .required(false)
            .multiple(false)
        )
        // OPTIONAL ARGUMENTS
        .arg(
            Arg::with_name("blur")
                .long("blur")
                .help("Apply blur to the image")
                .group("options")
                .validator(_validate_float)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("huerotate")
                .long("hue-rotate")
                .help("Rotate the hue color scale of the image")
                .group("options")
                .group("colors")
                .validator(_validate_integer)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("brighten")
                .long("brighten")
                .help("Sets the brighten of the image")
                .group("options")
                .validator(_validate_integer)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("contrast")
                .long("contrast")
                .help("Apply contrast to the image")
                .group("options")
                .validator(_validate_float)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("grayscale")
            .long("grayscale")
            .short("G")
            .help("Apply grayscale filter to the image")
            .group("options")
            .group("colors")
        )
        .arg(
            Arg::with_name("invert")
            .long("invert")
            .short("I")
            .help("Apply Invert colors filter")
            .group("colors")
            .group("options")
        )
}

fn _validate_integer(arg: String) -> Result<(), String> {
    match arg.parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Type Error: Integer expected"))
    }
}

fn _validate_float(arg: String) -> Result<(), String> {
    match arg.parse::<f32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Type Error: Float expected"))
    }
}

