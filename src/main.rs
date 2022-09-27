use base64::{decoder::Decoder, encoder::Encoder};
use clap::{arg, command, value_parser};
use std::env;

mod base64;

fn main() {
    let matches = command!()
        .global_setting(clap::AppSettings::DeriveDisplayOrder)
        .about("Encodes supplied [text] as a String into base64 encoded text.")
        .arg(arg!([text]))
        .arg(arg!(-d --decode <TEXT> "text to decode").required(false))
        .arg(
            arg!(-w --wrap <COLS> 
        "wrap encoded lines after COLS value (default 76).\nUse 0 to disable line wrapping.")
            .required(false)
            .default_value("76")
            .value_parser(value_parser!(u16).range(0..)),
        )
        .get_matches();

    let result = if let Some(text) = matches.get_one::<String>("text") {
        let cols = matches.get_one::<u16>("wrap").unwrap().to_owned();
        let encoder = Encoder::new(cols as usize);
        Ok(encoder.encode(text))
    } else if let Some(text) = matches.get_one::<String>("decode") {
        Decoder::decode(text)
    } else {
        Ok(String::new())
    };

    match result {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e),
    }
}
