extern crate alloc;

use alloc::borrow::Cow;

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

#[derive(Template)]
#[template(path = "page.html.askama")]
struct Extends<'a> {
    title: &'a str,
    visitor_count: u64,
}

#[derive(Template)]
#[template(source = "<ul>{% for value in &values %}<li>{{ value }}</li>{% endfor %}</ul>", ext = "html")]
struct StatementFor<'a> {
    values: Vec<&'a str>,
}

#[derive(Template)]
#[template(source = "{{ a | lower }} {{ b | lower }} {{ c | lower }} {{ d | lower }} {{ e | lower }}", ext = "html")]
struct FilterLower<'a> {
    a: &'a str,
    b: String,
    c: usize,
    d: Box<String>,
    e: Cow<'a, str>,
}

#[derive(Template)]
#[template(source = "{{ a | upper }} {{ b | upper }} {{ c | upper }} {{ d | upper }} {{ e | upper }}", ext = "html")]
struct FilterUpper<'a> {
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
        output.reserve(InlineText::SIZE_HINT);

        InlineText.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn inline_variable(&self, output: &mut Self::Output, text: &str) -> benchmark::Result {
        output.reserve(InlineVariable::SIZE_HINT);

        InlineVariable {
            text,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn extends(&self, output: &mut Self::Output, title: &str, visitor_count: u64) -> benchmark::Result {
        output.reserve(Extends::SIZE_HINT);

        Extends {
            title,
            visitor_count,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn statement_for(&self, output: &mut Self::Output, values: Vec<&str>) -> benchmark::Result {
        output.reserve(StatementFor::SIZE_HINT);

        StatementFor {
            values,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }
    
    fn filter_lower(&self, output: &mut Self::Output, a: &str, b: String, c: usize, d: Box<String>, e: Cow<str>) -> benchmark::Result {
        output.reserve(FilterLower::SIZE_HINT);

        FilterLower {
            a,
            b,
            c,
            d,
            e,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }
    
    fn filter_upper(&self, output: &mut Self::Output, a: &str, b: String, c: usize, d: Box<String>, e: Cow<str>) -> benchmark::Result {
        output.reserve(FilterUpper::SIZE_HINT);

        FilterUpper {
            a,
            b,
            c,
            d,
            e,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }
}
