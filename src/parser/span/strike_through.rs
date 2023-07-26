use parser::span::parse_spans;
use parser::{Span, Span::StrikeThrough};
use regex::Regex;

pub fn parse_strike_through(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        static ref STRIKE_THROUGH_WAVE :Regex = Regex::new(r"^~~(?P<text>.+?)~~").unwrap();
    }

    if STRIKE_THROUGH_WAVE.is_match(text) {
        let caps = STRIKE_THROUGH_WAVE.captures(text).unwrap();
        let t = caps.name("text").unwrap().as_str();
        return Some((StrikeThrough(parse_spans(t)), t.len() + 4));
    }
    
    None
}

#[cfg(test)]
mod test {
    use parser::Span::{Text, StrikeThrough};
    use super::parse_strike_through;

    #[test]
    fn finds_strong() {
        assert_eq!(parse_strike_through("~~testing things~~ test"),
                   Some((StrikeThrough(vec![Text("testing things".to_owned())]), 18)));

        assert_eq!(parse_strike_through("~~w~~ things_ test"),
                   Some((StrikeThrough(vec![Text("w".to_owned())]), 5)));

        assert_eq!(parse_strike_through("~~w~~ things~~ test"),
                   Some((StrikeThrough(vec![Text("w".to_owned())]), 5)));

        assert_eq!(parse_strike_through("~~w~~_ testing things test"),
                   Some((StrikeThrough(vec![Text("w".to_owned())]), 5)));
    }
}