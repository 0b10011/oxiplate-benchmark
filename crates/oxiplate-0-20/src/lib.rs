extern crate alloc;

use alloc::borrow::Cow;
use oxiplate::prelude::*;

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

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{{ >a | lower }} {{ >b | lower }} {{ >c | lower }} {{ >d | lower }} {{ >e | lower }}")]
struct Lower<'a> {
    a: &'a str,
    b: String,
    c: usize,
    d: Box<String>,
    e: Cow<'a, str>,
}

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{{ >a | upper }} {{ >b | upper }} {{ >c | upper }} {{ >d | upper }} {{ >e | upper }}")]
struct Upper<'a> {
    a: &'a str,
    b: String,
    c: usize,
    d: Box<String>,
    e: Cow<'a, str>,
}

impl benchmark::Generator for Generator {
    type Output = String;

    #[inline]
    fn output(&self) -> Self::Output {
        String::new()
    }

    #[inline]
    fn inline_text(&self, output: &mut Self::Output) -> benchmark::Result {
        output.reserve(InlineText::ESTIMATED_LENGTH);

        InlineText.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn inline_variable(&self, output: &mut Self::Output, text: &str) -> benchmark::Result {
        output.reserve(InlineVariable::ESTIMATED_LENGTH);

        InlineVariable {
            text,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn extends(&self, output: &mut Self::Output, title: &str, visitor_count: u64) -> benchmark::Result {
        output.reserve(Extends::ESTIMATED_LENGTH);

        Extends {
            title,
            visitor_count,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn statement_for(&self, output: &mut Self::Output, values: Vec<&str>) -> benchmark::Result {
        output.reserve(StatementFor::ESTIMATED_LENGTH);

        StatementFor {
            values,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn filter_lower(&self, output: &mut Self::Output, a: &str, b: String, c: usize, d: Box<String>, e: Cow<str>) -> benchmark::Result {
        output.reserve(Lower::ESTIMATED_LENGTH);

        Lower {
            a,
            b,
            c,
            d,
            e,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn filter_upper(&self, output: &mut Self::Output, a: &str, b: String, c: usize, d: Box<String>, e: Cow<str>) -> benchmark::Result {
        output.reserve(Upper::ESTIMATED_LENGTH);

        Upper {
            a,
            b,
            c,
            d,
            e,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }
}
