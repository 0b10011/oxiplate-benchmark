use std::hint::black_box;

use benchmark::Generator;
use criterion::Criterion;

enum Part {
    Check,
    Bench,
}

/// Run benchmarks against selected packages.
pub fn run(c: &mut Criterion) {
    run_part(c, Part::Check);
    run_part(c, Part::Bench);
}

fn run_part(c: &mut Criterion, part: Part) {
    let mut generators: [(&str, &dyn Generator<Output = _>); _] = [
        #[cfg(feature = "other-libraries")]
        ("askama-latest", &askama_latest::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.1", &oxiplate_0_1::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.2", &oxiplate_0_2::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.3", &oxiplate_0_3::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.4", &oxiplate_0_4::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.5", &oxiplate_0_5::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.6", &oxiplate_0_6::Generator),
        #[cfg(feature = "recent")]
        ("oxiplate-0.7", &oxiplate_0_7::Generator),
        #[cfg(feature = "recent")]
        ("oxiplate-0.8", &oxiplate_0_8::Generator),
        ("oxiplate-0.9", &oxiplate_0_8::Generator),
    ];
    generators.reverse();

    macro_rules! bench {
        ($group_name:literal, $test_fn:ident, $expected_output:expr $(, $data:expr)*) => {
            match part {
                Part::Check => {
                    let expected_output = $expected_output;

                    for (package, generator) in generators {
                        let mut output = generator.output();
                        generator.$test_fn(&mut output, $($data),*);
                        let mut is_correct = false;
                        let output_string = String::from_utf8_lossy(output.as_bytes());
                        for expected in &expected_output {
                            if &output_string == expected {
                                is_correct = true;
                                break;
                            }
                        }
                        if !is_correct {
                            panic!(
                                "{package} generated the wrong output:\n===\n{}\n===\nexpected one of:\n===\n{}\n===",
                                output_string.replace("\n", "\\n"),
                                expected_output.map(|string| string.replace("\n", "\\n")).join("\n===\n")
                            );
                        }
                    }
                }
                Part::Bench => {
                    let mut group = c.benchmark_group($group_name);

                    for (package, generator) in generators {
                        group.bench_function(package, |b| {
                            b.iter(|| {
                                let mut output = black_box(generator.output());
                                generator.$test_fn(&mut output, $(black_box($data)),*);
                            })
                        });
                    }

                    drop(group);
                }
            }
        };
    }

    bench!("inline text", inline_text, ["hello world"]);
    bench!("inline variable", inline_variable, ["<text>"], "<text>");
    bench!(
        "extends",
        extends,
        [
            r#"<!DOCTYPE html>
<html dir="ltr">
<head>
    <meta charset="utf-8">
    <title>&lt;text></title>
</head>
<body>
    <h1 title="<text>">&lt;text></h1>
    <p>You're visitor #19!</p>
</body>
</html>
"#,
            // Oxiplate 0.2
            r#"<!DOCTYPE html>
<html dir="ltr">
<head>
    <meta charset="utf-8">
    <title>&lt;text></title>
</head>
<body>
    <h1 title="&lt;text>">&lt;text></h1>
    <p>You're visitor #19!</p>
</body>
</html>
"#,
            // Askama
            r#"<!DOCTYPE html>
<html dir="ltr">
<head>
    <meta charset="utf-8">
    <title>&#60;text&#62;</title>
</head>
<body>
    <h1 title="&#60;text&#62;">&#60;text&#62;</h1>
    <p>You're visitor #19!</p>
</body>
</html>
"#,
        ],
        "<text>",
        19
    );
    bench!(
        "for 10",
        statement_for,
        [format!("<ul>{}</ul>", "<li>hello world</li>".repeat(10))],
        ["hello world"].into_iter().cycle().take(10).collect()
    );
    bench!(
        "for 10,000",
        statement_for,
        [format!(
            "<ul>{}</ul>",
            "<li>hello world</li>".repeat(10_000)
        )],
        ["hello world"].into_iter().cycle().take(10_000).collect()
    );
    bench!(
        "for 10 disallowed chars",
        statement_for,
        [
            format!(
                "<ul>{}</ul>",
                r#"<li>&lt;script>alert("hi")&lt;/script></li>"#.repeat(10)
            ),
            format!(
                "<ul>{}</ul>",
                r#"<li>&#60;script&#62;alert(&#34;hi&#34;)&#60;/script&#62;</li>"#.repeat(10)
            ),
        ],
        [r#"<script>alert("hi")</script>"#]
            .into_iter()
            .cycle()
            .take(10)
            .collect()
    );
    bench!(
        "for 10,000 disallowed chars",
        statement_for,
        [
            format!(
                "<ul>{}</ul>",
                r#"<li>&lt;script>alert("hi")&lt;/script></li>"#.repeat(10_000)
            ),
            format!(
                "<ul>{}</ul>",
                r#"<li>&#60;script&#62;alert(&#34;hi&#34;)&#60;/script&#62;</li>"#.repeat(10_000)
            ),
        ],
        [r#"<script>alert("hi")</script>"#]
            .into_iter()
            .cycle()
            .take(10_000)
            .collect()
    );
}
