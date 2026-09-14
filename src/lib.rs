use std::hint::black_box;

use benchmark::Generator;
use criterion::Criterion;

enum Part {
    Check,
    Bench,
}

/// Run benchmarks against selected packages.
pub fn run(c: &mut Criterion) {
    let to_skip = run_part(c, Part::Check, vec![]);
    run_part(c, Part::Bench, to_skip);
}

fn run_part(
    c: &mut Criterion,
    part: Part,
    to_skip: Vec<(&str, &str)>,
) -> Vec<(&'static str, &'static str)> {
    let mut generators: [(&str, &dyn Generator<Output = _>); _] = [
        #[cfg(feature = "other-libraries")]
        ("askama-latest", &askama_latest::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.01", &oxiplate_0_1::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.02", &oxiplate_0_2::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.03", &oxiplate_0_3::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.04", &oxiplate_0_4::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.05", &oxiplate_0_5::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.06", &oxiplate_0_6::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.07", &oxiplate_0_7::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.08", &oxiplate_0_8::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.09", &oxiplate_0_9::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.10", &oxiplate_0_10::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.11", &oxiplate_0_11::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.12", &oxiplate_0_12::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.13", &oxiplate_0_13::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.14", &oxiplate_0_14::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.15", &oxiplate_0_15::Generator),
        #[cfg(feature = "all-packages")]
        ("oxiplate-0.16", &oxiplate_0_16::Generator),
        #[cfg(feature = "recent")]
        ("oxiplate-0.17", &oxiplate_0_17::Generator),
        #[cfg(feature = "recent")]
        ("oxiplate-0.18", &oxiplate_0_18::Generator),
        ("oxiplate-0.19", &oxiplate_0_19::Generator),
    ];
    generators.reverse();

    let mut skipped = vec![];

    macro_rules! bench {
        ($group_name:literal, $test_fn:ident, $expected_output:expr $(, $data:expr)*) => {
            match part {
                Part::Check => {
                    let expected_output = $expected_output;

                    for (package, generator) in generators {
                        let mut output = generator.output();
                        match generator.$test_fn(&mut output, $($data),*) {
                            benchmark::Result::Done => (),
                            benchmark::Result::ToDo => {
                                println!("`{}` is not written yet for {package}", $group_name);
                                skipped.push((package, $group_name));
                                continue;
                            },
                            benchmark::Result::Unimplemented => {
                                println!("`{}` is not implemented for {package}", $group_name);
                                skipped.push((package, $group_name));
                                continue;
                            },
                        }
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
                                "{package} generated the wrong output for `{}`:\n===\n{}\n===\nexpected one of:\n===\n{}\n===",
                                $group_name,
                                output_string.replace("\n", "\\n"),
                                expected_output.map(|string| string.replace("\n", "\\n")).join("\n===\n")
                            );
                        }
                    }
                }
                Part::Bench => {
                    let mut group = c.benchmark_group($group_name);

                    for (package, generator) in generators {
                        if to_skip.contains(&(package, $group_name)) {
                            println!("`{}` skipped for `{package}`", $group_name);
                            continue;
                        }

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
    bench!(
        "filter lower",
        filter_lower,
        ["hello world! мм цц"],
        "Hello World! Мм Цц"
    );
    bench!(
        "filter upper",
        filter_upper,
        ["HELLO WORLD! ММ ЦЦ"],
        "Hello World! Мм Цц"
    );

    skipped
}
