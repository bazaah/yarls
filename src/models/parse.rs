use crate::models::*;

named!(
    to_u32<u32>,
    map_res!(map_res!(nom::digit, std::str::from_utf8), |s: &str| s
        .parse::<u32>())
);

named!(
    to_string<String>,
    map_res!(
        map_res!(nom::alphanumeric, std::str::from_utf8),
        |s: &str| s.parse::<String>()
    )
);

named!(
    grab_verses<RangeOrNumber>,
    do_parse!(
        tag_s!("(")
            >> number: to_u32
            >> opt!(tag_s!(".."))
            >> range: opt!(to_u32)
            >> tag_s!(")")
            >> (RangeOrNumber::which((number, range)))
    )
);

named!(
    grab_header<String>,
    do_parse!(tag_s!("[") >> header: to_string >> tag_s!("]") >> (header))
);

named!(
    pub create_section<&[u8], LyricSection>,
    do_parse!(
        part: pair!(
            grab_header,
            grab_verses
        ) >>
        (LyricSection::new(part.0, part.1))
    )
);

named!(
    pub match_domain<&[u8], Option<SupportedDomain>>,
    do_parse!(
        alt!(tag_s!("https") | tag_s!("http")) >>
        tag_s!("://") >>
        opt!(tag_s!("www.")) >>
        domain: to_string >>
        tag_s!(".com") >> 
        (SupportedDomain::new(domain.as_str()))
    )
);

