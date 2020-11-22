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
                .args(&["blur", "huerotate", "brighten", "contrast"]),
        )
        // OPTIONAL ARGUMENTS
        .arg(
            Arg::with_name("blur")
                .long("blur")
                .takes_value(true)
                .help("Apply blur to the image"),
        )
        .arg(
            Arg::with_name("huerotate")
                .long("hue-rotate")
                .takes_value(true)
                .help("Rotate the hue color scale of the image"),
        )
        .arg(
            Arg::with_name("brighten")
                .long("brighten")
                .takes_value(true)
                .help("Sets the brighten of the image"),
        )
        .arg(
            Arg::with_name("contrast")
                .long("contrast")
                .takes_value(true)
                .help("Apply contrast to the image"),
        )
}
