use logos::{Lexer, Logos};
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

    #[regex(r#"(<p[^>]*>|</p>)[^<]*"#, |_| logos::Skip)]
    Paragraf,
    #[regex(r#"(<h1[^>]*>|</h1>)[^<]*"#, |_| logos::Skip)]
    Header,
    #[regex(r#"<head>([^<]|<meta|<title|</title)*</head>\s*"#, |_| logos::Skip)]
    Head,
    #[regex(r#"</html>\s*"#, |_| logos::Skip)]
    Html,
    #[regex(r#"<![^<]*<html>\s*"#, |_| logos::Skip)]
    Doctype,
    #[regex(r#"(<body[^>]*>|</body>)[^<]*"#, |_| logos::Skip)]
    Body,
    #[regex(r#"(<a\s*|</a\s*>[^<]*)"#, |_| logos::Skip)]
    A,
    #[regex(r#"name="[^"]*"\s*"#, |_| logos::Skip)]
    Name,
    #[regex(r#">[^<]*"#, |_| logos::Skip)]
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

    offset = url.find('"').unwrap() + 1;
    url.drain(..offset);

    offset = url.find('"').unwrap();
    let link_url = LinkUrl(url[..offset].to_string().clone());
    url.drain(..offset);

    offset = url.find('>').unwrap() + 1;
    url.drain(..offset);

    let link_text = LinkText(url[..].to_string().clone());

    (link_url,link_text)
}
