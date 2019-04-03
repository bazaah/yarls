use crate::{
    models::parse::{create_section, match_domain},
    scraping::*,
};
use std::{
    fmt::Write as fmtWrite,
    fs::OpenOptions as FileOpts,
    io::{self, BufRead, Write as ioWrite},
    ops::RangeInclusive as iRange,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

mod parse;

#[derive(EnumIter, Debug, Display)]
pub enum SupportedDomain {
    #[strum(serialize = "metrolyrics")]
    Metrolyrics,

    #[strum(serialize = "genius")]
    Genius,

    #[strum(serialize = "worshiptogether")]
    Worshiptogether,

    #[strum(serialize = "azlyrics")]
    Azlyrics,

    #[strum(serialize = "bethelmusic")]
    BethelMusic,

    #[strum(serialize = "hillsong")]
    Hillsong,
}

impl SupportedDomain {
    pub fn new(domain: &str) -> Option<Self> {
        match domain {
            "metrolyrics" => Some(SupportedDomain::Metrolyrics),
            "genius" => Some(SupportedDomain::Genius),
            "worshiptogether" => Some(SupportedDomain::Worshiptogether),
            "azlyrics" => Some(SupportedDomain::Azlyrics),
            "bethelmusic" => Some(SupportedDomain::BethelMusic),
            "hillsong" => Some(SupportedDomain::Hillsong),
            _ => None,
        }
    }

    pub fn scrape(&self, url: &str) -> Vec<String> {
        match self {
            SupportedDomain::Metrolyrics => from_metrolyrics(url),
            SupportedDomain::Genius => from_genius(url),
            SupportedDomain::Worshiptogether => from_worshiptogether(url),
            SupportedDomain::Azlyrics => from_azlyrics(url),
            SupportedDomain::BethelMusic => from_bethelmusic(url),
            SupportedDomain::Hillsong => from_hillsong(url),
        }
    }

    pub fn print_variants() -> String {
        let mut variants = String::from("Use a supported domain: ");
        for variant in Self::iter() {
            write!(&mut variants, "[{}.com] ", variant).unwrap();
        }

        variants
    }
}

pub struct Lyrics<'a> {
    song_name: Option<&'a str>,
    author: Option<&'a str>,
    sections: Vec<LyricSection>,
}

impl<'a> Lyrics<'a> {
    pub fn new(
        song_name: Option<&'a str>,
        author: Option<&'a str>,
        sections: Vec<LyricSection>,
    ) -> Self {
        Self {
            song_name,
            author,
            sections,
        }
    }

    pub fn compose<W: ioWrite>(&self, mut writer: W, lyric_list: Vec<(usize, &str)>) {
        match self.song_name {
            Some(name) => writeln!(&mut writer, "{}", name).unwrap(),
            None => (),
        }
        match self.author {
            Some(artist) => writeln!(&mut writer, "({})\n", artist).unwrap(),
            None => (),
        }

        for section in &self.sections {
            writeln!(&mut writer, "[{}]", section.header).unwrap();
            for (line, lyric) in &lyric_list {
                match section.words {
                    RangeOrNumber::Number(n) => match (line, lyric) {
                        (line, lyric) if *line == n as usize => {
                            writeln!(&mut writer, "{}", lyric).unwrap();
                        }
                        (_, _) => (),
                    },
                    RangeOrNumber::Range(ref r) => {
                        let range = iRange::from(r.clone());
                        for number in range {
                            match (line, lyric) {
                                (line, lyric) if *line == number as usize => {
                                    writeln!(&mut writer, "{}", lyric).unwrap();
                                }
                                (_, _) => (),
                            }
                        }
                    }
                }
            }
        }
        writeln!(&mut writer, "~~~").unwrap();
    }
}

#[derive(Debug)]
pub struct LyricSection {
    header: String,
    words: RangeOrNumber,
}

impl LyricSection {
    pub fn new(header: String, words: RangeOrNumber) -> Self {
        LyricSection { header, words }
    }
}

#[derive(Debug)]
pub enum RangeOrNumber {
    Number(u32),
    Range(iRange<u32>),
}

impl RangeOrNumber {
    pub fn which(variant: (u32, Option<u32>)) -> Self {
        match variant {
            (num, None) => RangeOrNumber::Number(num),
            (num, Some(range)) => RangeOrNumber::Range(iRange::new(num, range)),
        }
    }
}

pub fn get_user_schema() -> Vec<LyricSection> {
    println!(
        "\nPlease select and order the lyrics: [<identifier>](<line number or range>) []() ..."
    );
    let mut input: String;
    {
        let stdin = io::stdin();
        let mut usr_input = stdin.lock().lines();
        input = usr_input.next().unwrap().unwrap().to_owned();
    }

    input
        .trim()
        .split(' ')
        .map(|s| {
            let unwrap = create_section(s.as_bytes()).unwrap();
            unwrap.1
        })
        .collect::<Vec<LyricSection>>()
}

pub fn enumerated_lyrics(raw_lyrics: &Vec<String>) -> Vec<(usize, &str)> {
    raw_lyrics
        .iter()
        .enumerate()
        .map(|(line, verse)| {
            let trimmed = verse.trim();
            println!("#{:>2}, {}", line, trimmed);
            (line, trimmed)
        })
        .collect::<Vec<(usize, &str)>>()
}

pub fn get_writer(w: Option<&str>) -> Box<ioWrite> {
    match w {
        Some(file_name) => match FileOpts::new()
            .create(true)
            .append(true)
            .open(file_name)
            .ok()
        {
            Some(file) => Box::new(file),
            None => Box::new(std::io::stdout()),
        },
        None => Box::new(std::io::stdout()),
    }
}

pub fn scrape_from(url: &str) -> Vec<String> {
    let validator = match_domain(url.as_bytes()).unwrap();

    match validator.1 {
        Some(valid_domain) => valid_domain.scrape(url),
        None => panic!("Attempting to scrape from unsupported domain!"),
    }
}

pub fn valid_target_domain(url: &str) -> Result<(), String> {
    match match_domain(url.as_bytes()).unwrap() {
        (_, Some(_)) => Ok(()),
        (_, None) => Err(SupportedDomain::print_variants()),
    }
}
