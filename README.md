# Oxiplate benchmarks

This repository contains benchmarks
that can be run against multiple versions of [Oxiplate](https://0b10011.io/oxiplate/),
as well as similar template engines.

`cargo bench` will run against recent versions of Oxiplate
and latest versions of similar template engines.
Use `cargo bench --features all-packages` to run against all supported versions.

## Adding a new package

From `/crates/`, run `cargo new --lib --name PACKAGE-MAJOR-MINOR`
where `PACKAGE` is the name of the package,
`MAJOR` is the major version number,
and `MINOR` is the minor version number.
Multiple minor versions cause collisions,
hence the lack of a patch in the version.

For other libraries, `PACKAGE-latest` is typically used instead.

In `./Cargo.toml`, you'll need to add the following
to the `[dependencies]` section:

```toml
benchmark = { version = "*", path = "../benchmark" }
```

In `./lib.rs`,
a struct typically named `Generator` needs to be created
and made to implement `benchmark::Generator`.
The methods should be `#[inline]`.

Then the package needs added to the `[dependencies]` section
of the main package's `/Cargo.toml`,
as well as updating the features accordingly.

Once that's finished,
the package name and struct implementing `benchmark::Generator`
can be added to the `generators` array in the main `run()` function.

## Adding a new benchmark

Each benchmark is a method on the `benchmark::Generator` trait
with a single `bench!()` macro call
near the end of the main `run()` function.

Benchmarks should test:

1. One specific (related group of) feature(s); or
2. Real-world use cases

More specific benchmarks can be helpful
for catching regressions in specific parts of the codebase,
while more broad benchmarks
can help ensure Oxiplate remains performant for the real world.

The third argument is the expected output,
which is tested for all of the packages
before that set of benchmarks is run.
Most differences in output are unacceptable.
However, different escapes can sometimes be okay
assuming the meaning of the escapes is equivalent.
There is nothing in place to handle this currently,
but a replace before comparison will likely handle these.

There's also a possibility
that one of the template engines being benchmarked
doesn't support one or more of the features being checked.
There's nothing in place to support this quite yet,
but skipping a package for a specific benchmark
would be okay assuming it's known to not support the feature.
