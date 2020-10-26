use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub fn build_app() -> App<'static, 'static> {
    let app = clap::app_from_crate!()
        .arg(
            Arg::with_name("file")
            .help("Defines the file/path to use, can be a file or directory")
            .value_name("PATH(S)/FILE(S)")
            .multiple(true)
            .required(true)
		)
        .arg(
            Arg::with_name("recursive")
            .help("Search every subdirectory for images")
            .short("R")
            .long("recursive")
            .visible_alias("recurse")
        )
        .arg(
            Arg::with_name("benchmark")
            .help("Exits upon loading everything")
            .long_help(
                "Exits upon loading and initializing everything needed to start displaying the image on the screen. Use this option to get how long it took to load the image, initialize OpenGL, etc.",
                )
            .long("bench")
            .visible_alias("benchmark")
            .required(false),
        );

    app
}
