<div align="center">
  <h1>Rust Examples</h1>
  Collection of examples in <a href="https://www.rust-lang.org/">Rust</a> to show how to get things done in an idiomatic way
</div>

## Installing

You need to have [Rust] **v1.34.0** or higher installed to run the examples.

Some examples depend on external crates from [crates.io], but everything is taken care of by cargo, so you don't need to perform any additional steps.
Just clone the repository, using the following commands, and you are good to go.

```
$ git clone https://github.com/umut-sahin/rust-examples.git
$ cd rust-examples
```

## Building

You can use the following command to build every single example.

```
$ cargo build --all
```

Note that with this command cargo will download and compile every additional package for every example.
So it may take some time.

## Running a specific example

```
$ cargo run -q --package <example-name>
```

Also, you can pass arguments like so

```
$ cargo run -q --package <example-name> -- [arguments...]
```

Note that there are some examples, which are in the form of a library, therefore, does not come with an executable.
For those examples, you may use the test command like so

```
$ cargo test -q --package <example-name>
```

## Available examples

- [brainfuck-interpreter]
- [command-line-arguments]
- [memory-mapped-io]
- [operator-overloading]
- [parking-threads]
- [read-from-console]
- [threads]
- [write-to-console]

## Changelog

See [CHANGELOG].

## License

This repository is released under [MIT license].
Please see [LICENSE] for details.

## Contributing

Contributions are welcome!
Please see [CONTRIBUTING] for guidance.

## Authors

See [AUTHORS].


[//]: # (Links)

[AUTHORS]:
  https://github.com/umut-sahin/rust-examples/blob/master/AUTHORS.md
[brainfuck-interpreter]:
  https://github.com/umut-sahin/rust-examples/tree/master/brainfuck-interpreter
[CHANGELOG]:
  https://github.com/umut-sahin/rust-examples/blob/master/CHANGELOG.md
[command-line-arguments]:
  https://github.com/umut-sahin/rust-examples/tree/master/command-line-arguments
[CONTRIBUTING]:
  https://github.com/umut-sahin/rust-examples/blob/master/CONTRIBUTING.md
[crates.io]:
  https://crates.io/
[LICENSE]:
  https://github.com/umut-sahin/rust-examples/blob/master/LICENSE
[memory-mapped-io]:
  https://github.com/umut-sahin/rust-examples/tree/master/memory-mapped-io
[MIT License]:
  https://tldrlegal.com/license/mit-license
[operator-overloading]:
  https://github.com/umut-sahin/rust-examples/tree/master/operator-overloading
[parking-threads]:
  https://github.com/umut-sahin/rust-examples/tree/master/parking-threads
[read-from-console]:
  https://github.com/umut-sahin/rust-examples/tree/master/read-from-console
[Rust]:
  https://www.rust-lang.org/
[threads]:
  https://github.com/umut-sahin/rust-examples/tree/master/threads
[write-to-console]:
  https://github.com/umut-sahin/rust-examples/tree/master/write-to-console
