<div align="center">
  <h1>threads</h1>
  A simple program to demonstrate how to create threads, how to pass data to threads, how to wait for threads to finish and how to return data from threads
</div>

## Authors

- [umut-sahin](https://github.com/umut-sahin) - Umut Şahin \<umutsahin@protonmail.com>

## Prerequisites

- [write-to-console] example.

## Description

This program uses the [**thread**] module from the standard library.

In the [**thread**] module, there is a public function named [**spawn**] which takes a closure and creates an OS thread executing that closure.
Upon creating the thread, [**spawn**] immediately returns to the caller, so the parent thread is not blocked.
[**spawn**] returns a generic struct [**JoinHandle<**T**>**], which is used to wait for the created thread to finish.

Upon creating the new thread, the main thread writes **Hello from the main thread!** to the standard output.
The main thread then starts waiting for the new thread to finish using the [**join**] method on the [**JoinHandle<**T**>**] returned from the [**spawn**] call at the beginning of the program.
Meanwhile, the new thread is executing the closure that we passed while calling the [**spawn**] function.

This closure is marked with **move** because otherwise **data_to_pass** would have been captured by reference.
This is a huge problem since, theoretically, the new thread may outlive the creator thread, which could make captured reference invalid.

Anyway, the closure that we passed while calling the [**spawn**] function writes **Hello from the new thread!** to the standard output then returns **data_to_pass + 42**.

Now the main thread can continue its execution.
The [**join**] function returns the result of the closure to the main thread, which is **data_to_pass + 42** in this case.

Then, we assert that the **threads_return** is indeed **data_to_pass + 42** and we are done.

## Usage

```
$ cargo run -q --package threads
```

## Arguments and flags

None

## Example runs

```
$ cargo run -q --package threads
Hello from the main thread!
Hello from the new thread!
```

```
$ cargo run -q --package threads
Hello from the new thread!
Hello from the main thread!
```

## Known bugs

None

## Limitations

None

## Notes

- The [**spawn**] function panics if the OS fails to create the thread.
  If explicit control over the possible errors is required, [**std::thread::Builder**] can be used.

- Usage of the [**unwrap**] method on [**join**]'s result was on purpose.
  This is because [**join**] will only result in [**Err**] if the new thread panics and, while it is possible for the new thread to panic due to [**println!**], it is highly unlikely that the panic will occur.

## Further reading

- [Using Threads to Run Code Simultaneously]
- [Safe Concurrency with Rust]
- [Where do Rust threads come from?]


[//]: # (Links)

[**Err**]:
  https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
[**join**]:
  https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join
[**JoinHandle<**T**>**]:
  https://doc.rust-lang.org/std/thread/struct.JoinHandle.html
[**println!**]:
  https://doc.rust-lang.org/std/macro.println.html
[**spawn**]:
  https://doc.rust-lang.org/std/thread/fn.spawn.html
[**std::thread::Builder**]:
  https://doc.rust-lang.org/std/thread/struct.Builder.html
[**thread**]:
  https://doc.rust-lang.org/std/thread/
[**unwrap**]:
  https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
[Safe Concurrency with Rust]:
  http://squidarth.com/rc/rust/2018/06/04/rust-concurrency.html
[Using Threads to Run Code Simultaneously]:
  https://doc.rust-lang.org/book/ch16-01-threads.html
[Where do Rust threads come from?]:
  http://squidarth.com/rc/rust/concurrency/2018/06/09/rust-threads-detach.html
[write-to-console]:
  https://github.com/umut-sahin/rust-examples/tree/master/write-to-console
