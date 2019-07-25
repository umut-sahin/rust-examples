<div align="center">
  <h1>write-to-console</h1>
  A simple program which demonstrates how to write to the standard output
</div>

## Authors

- [umut-sahin](https://github.com/umut-sahin) - Umut Şahin \<umutsahin@protonmail.com>

## Prerequisites

None

## Description

This program uses a single [**println!**] macro invocation to write "Hello World!" to the standard output.
Nothing fancy.

## Usage

```
$ cargo run -q --package write-to-console
```

## Arguments and flags

None

## Example run

```
$ cargo run -q --package write-to-console
Hello World!
```

## Known bugs

None

## Limitations

None

## Notes

- [**println!**] macro panics if writing to the standard output fails.
- While using [**println!**] macro is convenient, each [**println!**] invocation locks the mutex for the standard output.
  So if the application is performance critical, using [**println!**] should be avoided.

## Further reading

None


[//]: # (Links)

[**println!**]:
  https://doc.rust-lang.org/std/macro.println.html
