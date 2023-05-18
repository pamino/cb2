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
    #[regex(r#"href=[^>]*>[^<]*"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex(r#"(<p[^>]*>|</p>)[^<]*"#)]
    Paragraf,
    #[regex(r#"(<h1[^>]*>|</h1>)[^<]*"#)]
    Header,
    #[regex(r#"<head>([^<]|<meta|<title|</title)*</head>\s*"#)]
    Head,
    #[regex(r#"</html>\s*"#)]
    Html,
    #[regex(r#"<![^<]*<html>\s*"#)]
    Doctype,
    #[regex(r#"(<body[^>]*>|</body>)[^<]*"#)]
    Body,
    #[regex(r#"(<a\s*|</a\s*>[^<]*)"#)]
    A,
    #[regex(r#"name="[^"]*"\s*"#)]
    Name,
    #[regex(r#">[^<]*"#)]
    Rest,

    // Catch any error
    #[error]
    Error,

}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let mut url = String::from(lex.slice());

    let mut offset = url.find("href").unwrap();
    url.drain(..offset);

    let mut offset = url.find('"').unwrap() + 1;
    url.drain(..offset);

    offset = url.find('"').unwrap();
    let linkUrl = LinkUrl(url[..offset].to_string().clone());
    url.drain(..offset);

    offset = url.find('>').unwrap() + 1;
    url.drain(..offset);

    let linkText = LinkText(url[..].to_string().clone());

    (linkUrl,linkText)
}
