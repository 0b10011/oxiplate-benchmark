use oxiplate::{prelude::*};

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
#[oxiplate_inline(html: "{{ >value | lower }}")]
struct Lower<'a> {
    value: &'a str,
}

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{{ >value | upper }}")]
struct Upper<'a> {
    value: &'a str,
}

impl benchmark::Generator for Generator {
    type Output = String;

    #[inline]
    fn output(&self) -> Self::Output {
        String::new()
    }

    #[inline]
    fn inline_text(&self, output: &mut Self::Output) -> benchmark::Result {
        InlineText.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn inline_variable(&self, output: &mut Self::Output, text: &str) -> benchmark::Result {
        InlineVariable {
            text,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn extends(&self, output: &mut Self::Output, title: &str, visitor_count: u64) -> benchmark::Result {
        Extends {
            title,
            visitor_count,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn statement_for(&self, output: &mut Self::Output, values: Vec<&str>) -> benchmark::Result {
        StatementFor {
            values,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn filter_lower(&self, output: &mut Self::Output, value: &str) -> benchmark::Result {
        Lower {
            value,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }

    #[inline]
    fn filter_upper(&self, output: &mut Self::Output, value: &str) -> benchmark::Result {
        Upper {
            value,
        }.render_into(output).unwrap();

        benchmark::Result::Done
    }
}
