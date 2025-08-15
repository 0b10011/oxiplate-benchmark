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

#[derive(Oxiplate)]
#[oxiplate = "page.html.oxip"]
struct Extends<'a> {
    title: &'a str,
    visitor_count: u64,
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

    #[inline]
    fn extends(&self, output: &mut Self::Output, title: &str, visitor_count: u64) {
        Extends {
            title,
            visitor_count,
        }.render(output).unwrap();
    }
}
