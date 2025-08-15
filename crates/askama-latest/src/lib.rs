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
        InlineText.render_into(output).unwrap();
    }

    #[inline]
    fn inline_variable(&self, output: &mut Self::Output, text: &str) {
        InlineVariable {
            text,
        }.render_into(output).unwrap();
    }
}
