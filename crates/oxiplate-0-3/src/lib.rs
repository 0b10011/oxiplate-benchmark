extern crate alloc;

use alloc::borrow::Cow;
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

#[derive(Oxiplate)]
#[oxiplate = "page.html.oxip"]
struct Extends<'a> {
    title: &'a str,
    visitor_count: u64,
}

#[derive(Oxiplate)]
#[oxiplate_inline = "<ul>{% for value in &values %}<li>{{ value }}</li>{% endfor %}</ul>"]
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
        output.write_str(&format!("{}", InlineText)).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn inline_variable(&self, output: &mut Self::Output, text: &str) -> benchmark::Result {
        output.write_str(&format!("{}", InlineVariable {
            text,
        })).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn extends(&self, output: &mut Self::Output, title: &str, visitor_count: u64) -> benchmark::Result {
        output.write_str(&format!("{}", Extends {
            title,
            visitor_count,
        })).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn statement_for(&self, output: &mut Self::Output, values: Vec<&str>) -> benchmark::Result {
        output.write_str(&format!("{}", StatementFor {
            values,
        })).unwrap();

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
