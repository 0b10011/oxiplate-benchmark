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
    fn inline_text(&self, output: &mut Self::Output) {
        InlineText.render_into(output).unwrap()
    }

    #[inline]
    fn inline_variable(&self, output: &mut Self::Output, text: &str) {
        InlineVariable {
            text,
        }.render_into(output).unwrap()
    }

    #[inline]
    fn extends(&self, output: &mut Self::Output, title: &str, visitor_count: u64) {
        Extends {
            title,
            visitor_count,
        }.render_into(output).unwrap();
    }

    #[inline]
    fn statement_for(&self, output: &mut Self::Output, values: Vec<&str>) {
        StatementFor {
            values,
        }.render_into(output).unwrap();
    }
}
