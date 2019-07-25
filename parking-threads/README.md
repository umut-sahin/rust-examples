<div align="center">
  <h1>parking-threads</h1>
  A simple program to demonstrate how to put threads to sleep and wake them on demand
</div>

## Authors

- [umut-sahin](https://github.com/umut-sahin) - Umut Şahin \<umutsahin@protonmail.com>

## Prerequisites

- [threads] example.
- [write-to-console] example.

## Description

In our example, the main thread spawns a new thread.
Throughout the execution of the program, both the main thread and the new thread prints what they are doing.

Upon spawning the new thread, the main thread puts itself to sleep for 1 second.
This can be done with the public [**sleep**] function in the standard [**thread**] module.

Meanwhile, our new thread is putting itself to sleep with the public [**park**] function in the standard [**thread**] module.

In Rust, every thread has a unique [**Thread**] struct associated with it.
This struct contains various synchronization primitives to make parking threads this easy and, maybe even more importantly, impossible to misuse.

So now both of our threads are sleeping.

After one second, the main thread awakes, and it has the [**JoinHandle<**T**>**] for the new thread.
The main thread then uses the [**thread**] method of this [**JoinHandle<**T**>**] to get the unique [**Thread**] instance associated with the new thread and use its [**unpark**] method to wake up the new thread.

Now both of our threads are awake.
We wait for the new thread to finish, and we are done.

## Usage

```
$ cargo run -q --package parking-threads
```

## Arguments and flags

None

## Example run

```
$ cargo run -q --package parking-threads
Main thread is starting...
Main thread is going to sleep for 1 seconds...
New thread is starting...
New thread is parking...
Main thread is awaken...
Main thread is unparking the new thread...
Main thread is waiting for the new thread...
New thread is unparked...
New thread is ending...
Main thread is ending...
```

## Known bugs

None

## Limitations

None

## Notes

- I highly recommend reading the official [park and unpark] documentation.
  It is truly an incredible source for learning more.

## Further reading

- [park and unpark]


[//]: # (Links)

[**JoinHandle<**T**>**]:
  https://doc.rust-lang.org/std/thread/struct.JoinHandle.html
[**park**]:
  https://doc.rust-lang.org/std/thread/fn.park.html
[**sleep**]:
  https://doc.rust-lang.org/std/thread/fn.sleep.html
[**thread**]:
  https://doc.rust-lang.org/std/thread/index.html
[**Thread**]:
  https://doc.rust-lang.org/std/thread/struct.Thread.html
[**unpark**]:
  https://doc.rust-lang.org/std/thread/struct.Thread.html#method.unpark
[park and unpark]:
  https://doc.rust-lang.org/std/thread/fn.park.html#park-and-unpark
[threads]:
  https://github.com/umut-sahin/rust-examples/tree/master/threads
[write-to-console]:
  https://github.com/umut-sahin/rust-examples/tree/master/write-to-console
