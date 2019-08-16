<div align="center">
  <h1>read-from-console</h1>
  A simple program, which demonstrates how to read from the standard input
</div>

## Authors

- [umut-sahin](https://github.com/umut-sahin) - Umut Şahin \<umutsahin@protonmail.com>

## Prerequisites

- [write-to-console] example.

## Description

This program first asks for an integer kindly.
The standard output is line buffered by default in Rust.
This might cause the ```print!("+ ");``` to not print the **+ ** until a newline character is written to the standard output.
That's why the program uses the [**flush**] method on the [**Write**] trait which is [implemented][impl Write for Stdout] by [**Stdout**] struct ([**Stdout**] instance for the current process can be obtained with the [**stdout**] function in the standard [**io**] module).
The [**flush**] method flushes the content of the file buffer to the actual file unconditionally and immediately.
The [**unwrap**] method is used on the result of the flush operation because it's a critical error also it's not likely to occur.

The program then waits for the user input.
This can be done with the [**read_line**] method on the [**Stdin**] struct ([**Stdin**] instance for the current process can be obtained with the [**stdin**] function in the standard [**io**] module).
The [**read_line**] method reads data to a [**String**] instance from the standard input until a newline character is encountered.

[**String**]s automatically grow the on the heap, so we don't need to worry about the memory issues.

However, we might run into other issues such as the input not being a valid UTF-8, or we might encounter a critical I/O error.
In our example, the [**unwrap**] method is used to handle those errors.
It's not a wise thing to use [**unwrap**], but in this case, it's fine since we are aware of the consequences.

At this point, we have a valid UTF-8 encoded [**String**] instance with a newline character at the end, and we need to convert it to an integer.

To achieve this, first, we trim the [**String**] instance with the [**trim**] method of the primitive type [**str**] (It is possible to call the [**trim**] method of the primitive type [**str**] on a [**String**] instance because the [**Deref**] trait is [implemented][impl Deref for String] by [**String**]struct with **Target** = [**str**]).
Then we used the [**parse**] method on the [**string slice**] returned from the [**trim**] method to parse the string to an [**i32**].

The strange notation while calling the [**parse**] method is called the [Turbofish] syntax.
Basically, it is used to denote the type we are trying to parse the [**string slice**] to.

Since [**parse**] might fail, it returns a [**Result**] instance.

If this instance has [**Err**] as its discriminant, it means that the user gave us bad input.
In this case, we respond with the message: **"You did not enter an integer. That was not nice."**.
Otherwise, the user is co-operative, so we reply with: **"You entered ${enteredInteger}. Thank you for your cooperation."**.

## Usage

```
$ cargo run -q --package read-from-console
```

## Arguments and flags

None

## Example runs

```
$ cargo run -q --package read-from-console
- Could you enter an integer, please?
+ 42
- You entered 42. Thank you for your cooperation.
```

```
$ cargo run -q --package read-from-console
- Could you enter an integer, please?
+ Hello World!
- You did not enter an integer. That was not nice.
```

```
$ cargo run -q --package read-from-console
- Could you enter an integer, please?
+ 42Hello World!
- You did not enter an integer. That was not nice.
```

```
$ cargo run -q --package read-from-console
- Could you enter an integer, please?
+ 42.42
- You did not enter an integer. That was not nice.
```

## Known bugs

- [**parse**] is a really well-taught method for converting strings to integers.
  It not only checks the convertibility of the given string to an integer but also checks the overflow conditions.
  This is a wonderful thing.
  However, in our case, we respond with **"You did not enter an integer. That was not nice."** no matter what the error was.

## Limitations

None

## Notes

- Newline character is always **\n** in [Rust]s standard library.
- [**std::process::exit**] immediately terminates the process without any clean up.
  Thus, the variable **input**, is manually dropped before calling [**std::process::exit**].

## Further reading

None


[//]: # (Links)

[**Deref**]:
  https://doc.rust-lang.org/std/ops/trait.Deref.html
[**Err**]:
  https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
[**flush**]:
  https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush
[**i32**]:
  https://doc.rust-lang.org/std/primitive.i32.html
[**io**]:
  https://doc.rust-lang.org/std/io/index.html
[**parse**]:
  https://doc.rust-lang.org/std/primitive.str.html#method.parse
[**read_line**]:
  https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
[**Result**]:
  https://doc.rust-lang.org/std/result/enum.Result.html
[**std::process::exit**]:
  https://doc.rust-lang.org/std/process/fn.exit.html
[**stdin**]:
  https://doc.rust-lang.org/std/io/fn.stdin.html
[**Stdin**]:
  https://doc.rust-lang.org/std/io/struct.Stdin.html
[**stdout**]:
  https://doc.rust-lang.org/std/io/fn.stdout.html
[**Stdout**]:
  https://doc.rust-lang.org/std/io/struct.Stdout.html
[**str**]:
  https://doc.rust-lang.org/std/primitive.str.html
[**String**]:
  https://doc.rust-lang.org/std/string/struct.String.html
[**string slice**]:
  https://doc.rust-lang.org/std/primitive.str.html
[**trim**]:
  https://doc.rust-lang.org/std/primitive.str.html#method.trim
[**unwrap**]:
  https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
[**Write**]:
  https://doc.rust-lang.org/std/io/trait.Write.html
[impl Deref for String]:
  https://doc.rust-lang.org/std/string/struct.String.html#impl-Deref
[impl Write for Stdout]:
  https://doc.rust-lang.org/std/io/struct.Stdout.html#impl-Write
[Rust]:
  https://www.rust-lang.org/
[Turbofish]:
  https://turbo.fish/
[write-to-console]:
  https://github.com/umut-sahin/rust-examples/tree/master/write-to-console
