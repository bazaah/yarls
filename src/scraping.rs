use reqwest;
use scraper::{Html, Selector};
use selectors::attr::CaseSensitivity as SelCaseSens;

pub fn from_metrolyrics(song: &str) -> Vec<String> {
    let mut request = reqwest::get(song).unwrap();
    let raw = request.text().unwrap();
    let doc = Html::parse_document(&raw);
    let selector = Selector::parse("#lyrics-body-text > p").unwrap();
    let mut lyrics: Vec<_> = Vec::new();

    for lyric in doc.select(&selector) {
        let mut line = lyric.text().map(|s| s.to_owned()).collect::<Vec<_>>();
        lyrics.append(&mut line);
        lyrics.push(" ".to_string());
    }

    lyrics
}

pub fn from_genius(song: &str) -> Vec<String> {
    let mut request = reqwest::get(song).unwrap();
    let raw = request.text().unwrap();
    let doc = Html::parse_document(&raw);
    let selector = Selector::parse(".lyrics p").unwrap();
    let mut lyrics: Vec<_> = Vec::new();

    for lyric in doc.select(&selector) {
        let mut line = lyric.text().map(|s| s.to_owned()).collect::<Vec<_>>();
        lyrics.append(&mut line);
    }

    lyrics
}

pub fn from_azlyrics(song: &str) -> Vec<String> {
    let mut request = reqwest::get(song).unwrap();
    let raw = request.text().unwrap();
    let doc = Html::parse_document(&raw);
    let selector = Selector::parse("b + br + br + div").unwrap();
    let mut lyrics: Vec<_> = Vec::new();

    for lyric in doc.select(&selector) {
        let mut line = lyric.text().map(|s| s.to_owned()).collect::<Vec<_>>();
        lyrics.append(&mut line);
    }

    lyrics
}

pub fn from_hillsong(song: &str) -> Vec<String> {
    let mut request = reqwest::get(song).unwrap();
    let raw = request.text().unwrap();
    let doc = Html::parse_document(&raw);
    let selector = Selector::parse(".rowtext > p").unwrap();
    let mut lyrics: Vec<_> = Vec::new();

    for lyric in doc.select(&selector) {
        let mut line = lyric.text().map(|s| s.to_owned()).collect::<Vec<_>>();
        lyrics.append(&mut line);
    }

    lyrics
}

pub fn from_bethelmusic(song: &str) -> Vec<String> {
    let mut request = reqwest::get(song).unwrap();
    let raw = request.text().unwrap();
    let doc = Html::parse_document(&raw);
    let selector = Selector::parse(".content > p").unwrap();
    let mut lyrics: Vec<_> = Vec::new();

    for lyric in doc.select(&selector) {
        let lyric_set = lyric
            .children()
            .filter_map(|s| s.value().as_text())
            .map(|s| s.to_string())
            .collect::<String>();

        let mut line = lyric_set
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        lyrics.append(&mut line);
    }

    lyrics
}

pub fn from_worshiptogether(song: &str) -> Vec<String> {
    let mut request = reqwest::get(song).unwrap();
    let raw = request.text().unwrap();
    let doc = Html::parse_document(&raw);
    let selector = Selector::parse(".chord-pro-disp > div").unwrap();
    let outer_select = Selector::parse(".chord-pro-disp").unwrap();
    let inner_select = Selector::parse(".chord-pro-lyric").unwrap();
    let mut lyrics: Vec<_> = Vec::new();

    for container in doc.select(&outer_select) {
        for div in container.select(&selector) {
            if div
                .value()
                .has_class("chord-pro-br", SelCaseSens::AsciiCaseInsensitive)
            {
                lyrics.push(" ".to_string());
            } else if div
                .value()
                .has_class("chord-pro-line", SelCaseSens::AsciiCaseInsensitive)
            {
                let mut lyric = String::new();
                for lyric_fragment in div.select(&inner_select) {
                    let fragment = lyric_fragment.text().collect::<String>();
                    lyric.push_str(fragment.as_str());
                }
                lyrics.push(lyric);
            }
        }
    }

    lyrics
}
