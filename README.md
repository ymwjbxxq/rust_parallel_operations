# rust_parallel_operations

Thanks to Bryan Burgers from the Rustalan Slack channel, I got two solutions to run in parallel some operations:

- Using Tokio
- Using Futures

**Tokio**
I set up app_client in the lambda, and then, from a logical point of view, the app_client lives forever after that (across lambda invocations).
There's a lifetime for that: `'static`.
But from a Rust compiler point of view, it doesn't think that app_client has a `'static` lifetime. And tokio::spawn requires things to be static.
So one option is: to tell the compiler that `app_client is 'static`. And we can do that by leaking its memory (so it never gets cleaned up).

Another option is to avoid tokio::spawn, which is ultimately the source of your lifetime problems.

**Futures**
You can use `futures::join!` to run two (or more) futures at the same time, in parallel. And that’s usually better with lifetimes because the futures still run in the same scope and run to completion before the scope continues.

Always Bryan Burgers:
While it’s possible to configure `tokio` in different ways, I tend to think of `tokio::spawn` as sending the future to a different thread. So at this point, `tokio` needs to be very careful about all of the multi-threaded data-sharing aspects of Rust.
`futures::join!` runs things “on the same thread” conceptually:

- `join!(a, b)` is similar to `(a.await, b.await)`

So it doesn’t need to worry about the multi-threaded data-sharing aspects of Rust.