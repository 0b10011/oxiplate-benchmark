use std::fmt::Write;

use askama::Template;

pub struct Generator;

#[derive(Template)]
#[template(source = "hello world", ext = "html")]
struct InlineText;

#[derive(Template)]
#[template(source = "{{ text|safe }}", ext = "html")]
struct InlineVariable<'a> {
    text: &'a str,
}

impl benchmark::Generator for Generator {
    type Output = String;

    #[inline]
    fn output(&self) -> Self::Output {
        String::new()
    }

    #[inline]
    fn inline_text(&self, output: &mut Self::Output) {
        output.write_str(&format!("{}", InlineText)).unwrap();
    }

    #[inline]
    fn inline_variable(&self, output: &mut Self::Output, text: &str) {
        output.write_str(&format!("{}", InlineVariable {
            text,
        })).unwrap();
    }
}
