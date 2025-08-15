use oxiplate::{Oxiplate, Render};

pub struct Generator;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "hello world")]
struct InlineText;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{{ raw: text }}")]
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
        InlineText.render(output).unwrap()
    }

    #[inline]
    fn inline_variable(&self, output: &mut Self::Output, text: &str) {
        InlineVariable {
            text,
        }.render(output).unwrap()
    }
}
