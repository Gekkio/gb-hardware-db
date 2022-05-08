use anyhow::Error;
use clap::{value_t, App, Arg, ArgMatches};
use image::{imageops::FilterType, ImageOutputFormat};
use std::{
    fs::File,
    io::{self, Cursor, Write},
    u32,
};

fn run(matches: &ArgMatches) -> Result<(), Error> {
    let width = value_t!(matches, "width", u32).unwrap_or(u32::MAX);
    let height = value_t!(matches, "height", u32).unwrap_or(u32::MAX);
    let input = matches.value_of_os("INPUT").expect("Missing input file");
    let output = matches.value_of_os("output").expect("Missing output file");
    let img = image::open(&input)?.resize(width, height, FilterType::Lanczos3);
    let format = ImageOutputFormat::Jpeg(80);
    if output == "-" {
        let mut buffer = Cursor::new(Vec::new());
        img.write_to(&mut buffer, format)?;
        let mut w = io::stdout();
        w.write_all(buffer.get_ref())?;
    } else {
        let mut w = File::create(output)?;
        img.write_to(&mut w, format)?;
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let matches = App::new("gbhwdb-photo")
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .required(true)
                .help("Output file, or - to use standard output"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Input file, or - to use standard input")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("width")
                .short('w')
                .long("width")
                .value_name("WIDTH")
                .required_unless("height"),
        )
        .arg(
            Arg::with_name("height")
                .short('h')
                .long("height")
                .value_name("HEIGHT")
                .required_unless("width"),
        )
        .get_matches();
    run(&matches)
}
