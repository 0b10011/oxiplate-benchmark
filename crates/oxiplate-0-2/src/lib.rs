use std::fmt::Write;

use oxiplate::Oxiplate;

pub struct Generator;

#[derive(Oxiplate)]
#[oxiplate_inline = "hello world"]
struct InlineText;

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ raw: text }}"]
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
