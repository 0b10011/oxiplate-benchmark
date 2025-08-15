use std::hint::black_box;

use benchmark::Generator;
use criterion::Criterion;

/// Run benchmarks against selected packages.
pub fn run(c: &mut Criterion) {
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
        #[cfg(feature = "recent")]
        ("oxiplate-0.6", &oxiplate_0_6::Generator),
        #[cfg(feature = "recent")]
        ("oxiplate-0.7", &oxiplate_0_7::Generator),
        ("oxiplate-0.8", &oxiplate_0_8::Generator),
    ];
    generators.reverse();

    macro_rules! bench {
        ($group_name:literal, $test_fn:ident, $expected_output:literal $(, $data:expr)*) => {
            let mut group = c.benchmark_group($group_name);

            // Ensure the actual output is correct for each generator
            for (package, generator) in generators {
                let mut output = generator.output();
                generator.$test_fn(&mut output, $($data),*);
                assert_eq!(
                    String::from_utf8_lossy(output.as_bytes()),
                    $expected_output,
                    "{package} generated the wrong output"
                );
            }

            // Run benchmarks
            for (package, generator) in generators {
                group.bench_function(package, |b| {
                    b.iter(|| {
                        let mut output = black_box(generator.output());
                        generator.$test_fn(&mut output, $(black_box($data)),*);
                    })
                });
            }

            drop(group);
        };
    }

    bench!("inline text", inline_text, "hello world");
    bench!("inline variable", inline_variable, "<text>", "<text>");
}
