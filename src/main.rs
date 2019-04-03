#[macro_use]
extern crate nom;

use crate::models::{
    enumerated_lyrics, get_user_schema, get_writer, scrape_from, valid_target_domain, Lyrics,
};
use clap::{crate_authors, crate_version, App, Arg};
use std::io::{BufWriter, Write};

mod models;
mod scraping;

fn main() {
    let matches = App::new("yarls")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about("Yet Another Rust Lyric Scraper is a CLI utility for scraping song lyrics")
        .after_help("YARLS is designed with the expectation that the user will paste the absolute URL, \nE.g: yarls https://www.lyricwebsite.com/song-name")
        .arg(
            Arg::with_name("song")
                .takes_value(true)
                .required(true)
                .value_name("URL")
                .validator(|s: String| -> Result<(), String> { valid_target_domain(s.as_str()) })
                .help("Required URL"),
        )
        .arg(
            Arg::with_name("song_name")
                .takes_value(true)
                .value_name("NAME")
                .help("Optional song name"),
        )
        .arg(
            Arg::with_name("artist")
                .takes_value(true)
                .value_name("ARTIST")
                .help("Optional artist"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("PATH")
                .takes_value(true)
                .help("Create an output file, will append if file exists, otherwise defaults to stdout"),
        )
        .get_matches();

    let url = match matches.value_of("song") {
        Some(link) => link,
        None => unreachable!(),
    };
    let name = matches.value_of("song_name");
    let artist = matches.value_of("artist");
    let mut writer = BufWriter::new(get_writer(matches.value_of("output")));

    let lyrics = scrape_from(url);
    let numbered_lyrics = enumerated_lyrics(&lyrics);
    let schema = get_user_schema();

    let song = Lyrics::new(name, artist, schema);
    song.compose(writer.by_ref(), numbered_lyrics);
}
