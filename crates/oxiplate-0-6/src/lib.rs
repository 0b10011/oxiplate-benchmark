extern crate alloc;

use alloc::borrow::Cow;

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
    fn inline_text(&self, output: &mut Self::Output) -> benchmark::Result {
        InlineText.render(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn inline_variable(&self, output: &mut Self::Output, text: &str) -> benchmark::Result {
        InlineVariable {
            text,
        }.render(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn extends(&self, output: &mut Self::Output, title: &str, visitor_count: u64) -> benchmark::Result {
        Extends {
            title,
            visitor_count,
        }.render(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn statement_for(&self, output: &mut Self::Output, values: Vec<&str>) -> benchmark::Result {
        StatementFor {
            values,
        }.render(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn filter_lower(&self, _output: &mut Self::Output, _a: &str, _b: String, _c: usize, _d: Box<String>, _e: Cow<str>) -> benchmark::Result {
        benchmark::Result::Unimplemented
    }

    #[inline]
    fn filter_upper(&self, _output: &mut Self::Output, _a: &str, _b: String, _c: usize, _d: Box<String>, _e: Cow<str>) -> benchmark::Result {
        benchmark::Result::Unimplemented
    }
}
