<div align="center">
  <h1>command-line-arguments</h1>
  A simple program, which demonstrates how to iterate over the arguments, which are passed during the program invocation
</div>

## Authors

- [umut-sahin](https://github.com/umut-sahin) - Umut Şahin \<umutsahin@protonmail.com>

## Prerequisites

- [write-to-console] example.

## Description

This program uses the standard [**env**] module.
In the standard [**env**] module, there is a public function named [**args**], which returns an instance of [**Args**] struct.
We don't need to know the implementation detail of this struct.
All we need to know is that the [**Iterator**] trait is [implemented][impl Iterator for Args] by [**Args**] struct with **Item** = [**String**].
This iterator iterates over are the command line arguments, which are passed during the program invocation.

We consume this iterator and print the arguments along with the index of the argument.

To get the indexes along with the arguments, we used the [**enumerate**] method of the [**Iterator**] trait.
This method takes an iterator with **item = T** and converts it to a new iterator where **item = ([**usize**], T)**.
This new iterator yields the index of the item alongside with the item in the original iterator.

After every single argument is printed, the program finishes.

## Usage

```
$ cargo run -q --package command-line-arguments
```

```
$ cargo run -q --package command-line-arguments -- [...]
```

## Arguments and flags

- Argument: **...**,
  - Type: **Any**
  - Optional: **true**
  - Multiple: **true**

## Example runs

```
$ cargo run -q --package command-line-arguments
0: target/debug/command-line-arguments
```

```
$ cargo run -q --package command-line-arguments -- example "example with spaces" -e --example
0: target/debug/command-line-arguments
1: example
2: example with spaces
3: -e
4: --example
```

## Known bugs

None

## Limitations

None

## Notes

- Example runs are executed in a Linux environment.
  Their output may slightly differ if you are on a different platform.

## Further reading

None


[//]: # (Links)

[**args**]:
  https://doc.rust-lang.org/std/env/fn.args.html
[**Args**]:
  https://doc.rust-lang.org/std/env/struct.Args.html
[**enumerate**]:
  https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate
[**env**]:
  https://doc.rust-lang.org/std/env/
[**Iterator**]:
  https://doc.rust-lang.org/std/iter/trait.Iterator.html
[**String**]:
  https://doc.rust-lang.org/std/string/struct.String.html
[**usize**]:
  https://doc.rust-lang.org/std/primitive.usize.html
[impl Iterator for Args]:
  https://doc.rust-lang.org/std/env/struct.Args.html#impl-Iterator
[write-to-console]:
  https://github.com/umut-sahin/rust-examples/tree/master/write-to-console
