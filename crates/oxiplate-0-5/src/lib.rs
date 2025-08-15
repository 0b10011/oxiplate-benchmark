use std::fmt::Write;

use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "hello world")]
struct InlineText;

pub struct Generator;

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

#[derive(Oxiplate)]
#[oxiplate_inline(html: "<ul>{% for value in &values %}<li>{{ value }}</li>{% endfor %}</ul>")]
struct StatementFor<'a> {
    values: Vec<&'a str>,
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

    #[inline]
    fn extends(&self, output: &mut Self::Output, title: &str, visitor_count: u64) {
        output.write_str(&format!("{}", Extends {
            title,
            visitor_count,
        })).unwrap();
    }

    #[inline]
    fn statement_for(&self, output: &mut Self::Output, values: Vec<&str>) {
        output.write_str(&format!("{}", StatementFor {
            values,
        })).unwrap();
    }
}
