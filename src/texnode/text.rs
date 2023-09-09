use lazy_static::lazy_static;
use regex::Regex;

use super::span::Span;

lazy_static! {
    static ref TEXT_RE: Regex = Regex::new(r#"^[a-zA-Z0-9\s.,?!:;"'/@#&*+-=\(\)\[\]]+"#)
        .unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub span: Span,
    pub content: String
}

pub fn extract_text(tex: &str, start: usize) -> Option<Text> {
    if let Some(mat) = TEXT_RE.find(&tex[start..]) {
        Some(Text { 
            span: Span { start, end: start + mat.end() }, 
            content: mat.as_str().to_string()
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Text, Span, extract_text};

    const TEX: &'static str = r#"
\section{Introduction}
Hello, world!
\section{Theory}
An operator $T$ is \textbf{self-adjoint} if $T^\ast = T$, i.e.,
it is own adjoint.
"#;

    #[test]
    fn succeed_to_extract_texts() {
        let text = extract_text(TEX, 0).unwrap();
        assert_eq!(
            text, 
            Text {
                span: Span { start: 0, end: 1 },
                content: "\n".to_string()
            }
        );

        let text = extract_text(TEX, 24).unwrap();
        assert_eq!(
            text, 
            Text {
                span: Span { start: 24, end: 38 },
                content: "Hello, world!\n".to_string()
            }
        );

        let text = extract_text(TEX, 55).unwrap();
        assert_eq!(
            text, 
            Text {
                span: Span { start: 55, end: 67 },
                content: "An operator ".to_string()
            }
        );
    }

    #[test]
    fn fail_to_extract_texts() {
        assert_eq!(extract_text(TEX, 1), None);
    }

}
