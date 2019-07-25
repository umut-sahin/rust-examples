<div align="center">
  <h1>memory-mapped-io</h1>
  A simple program to demonstrate how to perform cross-platform <a href="https://en.wikipedia.org/wiki/Memory-mapped_I/O">Memory Mapped I/O</a> in Rust using the <a href="https://github.com/danburkert/memmap-rs">memmap</a> crate
</div>

## Authors

- [umut-sahin](https://github.com/umut-sahin) - Umut Şahin \<umutsahin@protonmail.com>

## Prerequisites

- [write-to-console] example.

## Description

[Memory Mapped I/O] is the act of mapping the memory and registers of an I/O device to the current processes address space.
Usually, [Memory Mapped I/O] is highly platform specific; but luckily there exist an awesome crate called [memmap], which makes writing this kind of example in a cross-platform way really easy.
In this example, we will only focus on [Memory Mapped File I/O].

[memmap] provides two structs, namely [**Mmap**] and [**MmapMut**].
The difference between them is pretty obvious.
For this example, we need to change the contents of the file **memory-mapped-io/assets/hello-world.txt**.
Thus we have used [**MmapMut**].

To create the [**MmapMut**] instance, we have used the [**map_mut**] function associated with the [**MmapMut**] struct ([**map_mut**] is unsafe because [Memory Mapped I/O] itself is unsafe).
This method has one parameter, which is a reference to a [**std::fs::File**] instance.
This instance can be obtained in various ways, which we will not cover.
You may check the standard library documentation for further information.
Let's go back to [**map_mut**].
[**map_mut**] method tries to map the given file into memory.
If the mapping is successful, [**map_mut**] method returns an [**std::io:Result**] instance with [**Ok**] discriminant.
If that's not the case, we terminate the program with an appropriate error message.

After obtaining a valid [**MmapMut**] instance, we are ready to process the file.
Since [**DerefMut**] trait is [implemented][impl DerefMut for MmapMut] by [**MmapMut**] struct with **Target** = [**&mut \[u8\]**], processing the file is super easy.
We just use a standard for loop like so ```for byte in &mut file_in_memory[..] { ... }```.
Here **file_in_memory** is first dereferenced into a **mutable u8 slice**.
Then we iterated over that slice (Notice ```byte: &mut u8```).
In this loop, we first print the ASCII character of the currently processed byte.
Then, if this byte is a valid ASCII lowercase or uppercase character, we change the case and update the byte in memory.

When this loop ends, the entire file is updated, so all alphabetic characters have had their cases changed.

Now we proceed with flushing the contents of the file back into the file system.
This can be done with the [**flush**] method of the [**MmapMut**] instance we have.
[**flush**] may fail, so it returns an [**std::io:Result**].
We check if the operation failed with an ```if let``` statement.
If it is failed, we print an appropriate error message and terminate the program.

If the program survived up to this point, it prints the new content of the file and then terminates peacefully.

## Usage

```
$ cargo run -q --package memory-mapped-io
```

## Arguments and flags

None

## Example runs

```
$ cargo run -q --package memory-mapped-io
Before: Hello World!
After: hELLO wORLD!
$ cargo run -q --package memory-mapped-io
Before: hELLO wORLD!
After: Hello World!
```

## Known bugs

None

## Limitations

None

## Notes

- [**process::exit**] function immediately terminates the current process without calling the destructors.
  So we need to release the resources we acquired manually before calling [**process::exit**].
  Unfortunately, we cannot use combinators such as [**unwrap_or_else**] since they require a closure to be passed, which would have captured the resources by moving.
  This is a problem because for example if we have used [**unwrap_or_else**] on the result of the [**map_mut**] with a closure, which captures **file**, we would not be able to [**drop**] the **file** in case of a [**flush**] failure.

- If you wonder why [**MmapMut**]**::**[**map_mut**] is unsafe, I highly recommend reading [How unsafe is mmap?] thread in the [Rust Internals] forum.

## Further reading

None


[//]: # (Links)

[**&mut \[u8\]**]:
  https://doc.rust-lang.org/nightly/std/primitive.slice.html
[**DerefMut**]:
  https://doc.rust-lang.org/nightly/core/ops/trait.DerefMut.html
[**drop**]:
  https://doc.rust-lang.org/std/mem/fn.drop.html
[**flush**]:
  https://docs.rs/memmap/0.7.0/memmap/struct.MmapMut.html#method.flush
[**map_mut**]:
  https://docs.rs/memmap/0.7.0/memmap/struct.MmapMut.html#method.map_mut
[**Mmap**]:
  https://docs.rs/memmap/0.7.0/memmap/struct.Mmap.html
[**MmapMut**]:
  https://docs.rs/memmap/0.7.0/memmap/struct.MmapMut.html
[**Ok**]:
  https://doc.rust-lang.org/nightly/std/result/enum.Result.html#variant.Ok
[**process::exit**]:
  https://doc.rust-lang.org/std/process/fn.exit.html
[**std::fs::File**]:
  https://doc.rust-lang.org/nightly/std/fs/struct.File.html
[**std::io:Result**]:
  https://doc.rust-lang.org/nightly/std/io/type.Result.html
[**unwrap_or_else**]:
  https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else
[How unsafe is mmap?]:
  https://users.rust-lang.org/t/how-unsafe-is-mmap/19635
[impl DerefMut for MmapMut]:
  https://docs.rs/memmap/0.7.0/memmap/struct.MmapMut.html#impl-DerefMut
[memmap]:
  https://github.com/danburkert/memmap-rs
[Memory Mapped File I/O]:
  https://en.wikipedia.org/wiki/Memory-mapped_file
[Memory Mapped I/O]:
  https://en.wikipedia.org/wiki/Memory-mapped_I/O
[Rust Internals]:
  https://users.rust-lang.org/
[write-to-console]:
  https://github.com/umut-sahin/rust-examples/tree/master/write-to-console
