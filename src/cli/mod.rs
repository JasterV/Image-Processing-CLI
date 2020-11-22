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
                .help("Sets the output path to store the image")
                .required(true)
                .index(2),
        )
        // OPTIONALS GROUP, AT LEAST ONE OPTION IS REQUIRED
        .group(ArgGroup::with_name("options").required(true).multiple(true))
        .group(ArgGroup::with_name("colors").required(false))
        // OPTIONAL ARGUMENTS
        .arg(
            Arg::with_name("blur")
                .long("blur")
                .help("Apply blur to the image")
                .group("options")
                .validator(_validate_f32)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("huerotate")
                .long("hue-rotate")
                .help("Rotate the hue color scale of the image")
                .group("options")
                .group("colors")
                .validator(_validate_i32)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("brighten")
                .long("brighten")
                .help("Sets the brighten of the image")
                .group("options")
                .validator(_validate_i32)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("contrast")
                .long("contrast")
                .help("Apply contrast to the image")
                .group("options")
                .validator(_validate_f32)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("crop")
                .long("crop")
                .help("Crop the image")
                .group("options")
                .takes_value(true)
                .number_of_values(4)
                .validator(_validate_u32),
        )
        .arg(
            Arg::with_name("rotate")
                .long("rotate")
                .takes_value(true)
                .possible_values(&["90", "180", "270"])
                .group("options"),
        )
        // FLAG ARGUMENTS
        .arg(
            Arg::with_name("grayscale")
                .long("grayscale")
                .short("G")
                .help("Apply grayscale filter to the image")
                .group("options")
                .group("colors"),
        )
        .arg(
            Arg::with_name("invert")
                .long("invert")
                .short("I")
                .help("Apply Invert colors filter")
                .group("colors")
                .group("options"),
        )
        .arg(
            Arg::with_name("flipv")
                .long("flipv")
                .help("Flip the image vertically")
                .group("options"),
        )
        .arg(
            Arg::with_name("fliph")
                .long("fliph")
                .help("Flip the image horizontally")
                .group("options"),
        )
}

fn _validate_u32(arg: String) -> Result<(), String> {
    match arg.parse::<u32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Type Error: Unsigned integer expected")),
    }
}

fn _validate_i32(arg: String) -> Result<(), String> {
    match arg.parse::<i32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Type Error: Integer expected")),
    }
}

fn _validate_f32(arg: String) -> Result<(), String> {
    match arg.parse::<f32>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Type Error: Float expected")),
    }
}
