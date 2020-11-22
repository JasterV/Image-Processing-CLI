use clap::{App, Arg};

const HELP_TEMPLATE: &str = "{bin} v{version}

-----------

{about}

{all-args}

-----------

(C) 2020 {author}

{after-help}";

/// This function builds the Command Line Application
/// already with its metadata, args boundaries etc.
pub fn build_cli() -> App<'static, 'static> {
    App::new("Image Editor application")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Edit, transform, crop, rotate etc. an image with just one command")
        .template(HELP_TEMPLATE)
        .before_help("Thanks for using this app bro :)")
        .after_help("Have a nice day!")
        .arg(Arg::with_name("FILE").index(1).required(true))
}
