#![allow(dead_code, unused_variables)]
#![allow(clippy::explicit_auto_deref)]

use std::fmt::Display;

#[derive(Debug)]
enum Error {
    Eof,
    Backtrack,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::Backtrack
    }
}

type Result<T> = std::result::Result<T, Error>;

trait Parser<T> {
    fn parse_next(&mut self, input: &mut &str) -> Result<T>;
}

impl<T, F: FnMut(&mut &str) -> Result<T>> Parser<T> for F {
    fn parse_next(&mut self, input: &mut &str) -> Result<T> {
        self(input)
    }
}

fn parse_while<T>(inp: &mut &str, mut p: impl Parser<T>) -> Vec<T> {
    let mut v = Vec::new();
    while let Ok(x) = p.parse_next(inp) {
        v.push(x)
    }

    v
}

fn parse_whileb(inp: &mut &str, mut p: impl FnMut(&char) -> bool) -> String {
    let mut v = Vec::new();
    for ch in inp.chars() {
        if p(&ch) {
            v.push(ch)
        } else {
            break;
        }
    }

    *inp = &inp[v.len()..];
    v.into_iter().collect::<String>()
}

fn parse_untilb(inp: &mut &str, mut p: impl FnMut(&char) -> bool) -> String {
    parse_whileb(inp, |ch| !p(ch))
}

fn token(inp: &mut &str) -> Result<String> {
    let output = parse_untilb(inp, |ch| ch.is_whitespace());
    Ok(output)
}

fn u64(inp: &mut &str) -> Result<u64> {
    let output = parse_whileb(inp, char::is_ascii_digit);
    Ok(output.parse::<u64>()?)
}

fn interleaved<T>(inp: &mut &str, mut p: impl Parser<T>, mut sep: impl Parser<()>) -> Vec<T> {
    let mut v: Vec<T> = Vec::new();

    while let Ok(x) = p.parse_next(inp) {
        v.push(x);
        if sep.parse_next(inp).is_err() {
            break;
        };
    }

    v
}

fn whitespace(inp: &mut &str) -> Result<()> {
    parse_whileb(inp, |ch| ch.is_whitespace());

    Ok(())
}

fn numbers(inp: &mut &str) -> Vec<u64> {
    interleaved(inp, u64, whitespace)
}

fn n(n: usize) -> impl Parser<String> {
    move |inp: &mut &str| {
        let backtrack = *inp;
        let mut i: usize = 0;
        let result_str = parse_whileb(inp, |ch| {
            i += 1;
            i <= n && !ch.is_whitespace()
        });

        if result_str.len() < n {
            *inp = backtrack;
            return Err(Error::Backtrack);
        }

        Ok(result_str)
    }
}

fn string(constant: &str) -> impl Parser<String> + use<'_> {
    let mut word = n(constant.len());
    move |inp: &mut &str| {
        let read = word.parse_next(inp)?;
        if read == constant {
            Ok(read)
        } else {
            Err(Error::Backtrack)
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_foo() {
        assert_eq!(u64(&mut "0").unwrap(), 0u64);
        assert_eq!(u64(&mut "18446744073709551615").unwrap(), !0u64);
        assert!(u64(&mut "18446744073709551616").is_err());
        assert_eq!(u64(&mut "1234").unwrap(), 1234u64);
        assert_eq!(
            numbers(&mut "123 456 789 0"),
            vec![123u64, 456u64, 789u64, 0u64]
        );

        assert_eq!(n(4).parse_next(&mut "123456").unwrap(), "1234".to_string());
        assert!(n(4).parse_next(&mut "123").is_err());

        assert_eq!(string("hello").parse_next(&mut "hello world").unwrap(), "hello");
    }
}
