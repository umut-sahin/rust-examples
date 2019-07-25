<div align="center">
  <h1>brainfuck-interpreter</h1>
  A simple <a href="https://esolangs.org/wiki/brainfuck">Brainfuck</a> interpreter written in Rust
</div>

## Authors

- [umut-sahin](https://github.com/umut-sahin) - Umut Şahin \<umutsahin@protonmail.com>

## Prerequisites

None

## Description

This program is made out of three parts.

The first part is to parse the command line options and to read the passed script file.
[clap] crate is used to parse the command line arguments and [memmap] crate is used to read the passed script file.

Upon reading file into memory, it's time to parse the script file.
This is done in the [**TryFrom**]<[**&\[u8\]**]> implementation for **Interpreter**.
The parser is a handwritten one.
It's pretty easy to read though.
If the parsing fails for some reason, a [**Result**]<**Interpreter**, **SyntaxError**> is returned with [**Err**] variant which contains the information about the error.

Otherwise, the parsing is successful and hence we got a nice **Interpreter** which contains a valid instruction list.

At this point, all we need to do left is to interpret the instructions.
This functionality is implemented for **Interpreter** in a method named **execute**.
**execute** might fail miserably though.
Cause of this could be an unexpected IO error or out of tape access.
Thus, there exists another type called **RuntimeError**.
If **execute** fails to interpret the whole program, it returns a **RuntimeError** instance wrapped inside of an [**Err**] variant.

Finally, we call the **execute** method on the **Interpreter** we have.
If **execute** returns an [**Err**], we simply print the information about the error and exit the program with an erroneous return code.

Otherwise, everything went as expected so we terminate the program normally.

## Usage

```
$ cargo run -q --package brainfuck-interpreter -- <SCRIPT>
```

## Arguments and flags

- Argument: **SCRIPT**,
  - Type: **Path**
  - Optional: **false**
  - Multiple: **false**

## Example runs

```
$ cargo run -q --package brainfuck-interpreter
error: The following required arguments were not provided:
    <SCRIPT>

USAGE:
    brainfuck-interpreter <SCRIPT>

For more information try --help
```

```
$ cargo run -q --package brainfuck-interpreter -- --help
brainfuck-interpreter 1.0.0
Umut S. <umutsahin@protonmail.com>
A simple Brainfuck interpreter written in Rust

USAGE:
    brainfuck-interpreter <SCRIPT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <SCRIPT>    Sets the script to execute
```

```
$ cargo run -q --package brainfuck-interpreter -- -V
brainfuck-interpreter 1.0.0
```

```
$ cargo run -q --package brainfuck-interpreter -- brainfuck-interpreter/assets/hello-world.bf
Hello World!
```

```
$ cargo run -q --package brainfuck-interpreter -- brainfuck-interpreter/assets/cat.bf
Hello World!
Hello World!
Goodbye World!
Goodbye World!
^D
```

```
$ cargo run -q --package brainfuck-interpreter -- brainfuck-interpreter/assets/rot13.bf
Hello World!
Uryyb Jbeyq!
^D
```

## Known bugs

None

## Limitations

None

## Notes

None

## Further reading

None


[//]: # (Links)
  
[**&\[u8\]**]:
  https://doc.rust-lang.org/nightly/std/primitive.slice.html
[**Err**]:
  https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
[**Result**]:
  https://doc.rust-lang.org/std/result/enum.Result.html
[**TryFrom**]:
  https://doc.rust-lang.org/std/convert/trait.TryFrom.html
[clap]:
  https://github.com/clap-rs/clap
[memmap]:
  https://github.com/danburkert/memmap-rs
