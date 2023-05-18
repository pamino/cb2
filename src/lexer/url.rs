use logos::{Lexer, Logos, Source};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    #[regex(r#"<a[^>]*href="[^"]*"[^>]*>[^<]*</a\s*>"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex(r#"\s*<p>.*</p>\s*"#, |_| logos::Skip)]
    Paragraf,
    #[regex(r#"\s*<h1>.*</h1>\s*"#, |_| logos::Skip)]
    Header,
    #[regex(r#"\s*<head>.*</head>\s*"#, |_| logos::Skip)]
    Head,
    #[regex(r#"</html>\s*"#, |_| logos::Skip)]
    Html,
    #[regex(r#"<![^<]*<html>"#, |_| logos::Skip)]
    Doctype,

    // Catch any error
    #[error]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let mut url = String::from(lex.slice());
    let mut offset = url.find('"').unwrap();
    url.drain(..offset);
    offset = url.find('"').unwrap() + 1;
    let linkUrl = LinkUrl(url[..offset].to_string().clone());
    url.drain(..offset);
    offset = url.find('>').unwrap() + 1;
    url.drain(..offset);
    offset = url.find('<').unwrap();
    let linkText = LinkText(url[..offset].to_string().clone());
    (linkUrl,linkText)
}
