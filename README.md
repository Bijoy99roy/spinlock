# SpinLock

This project implements a basic spinlock in Rust using atomics and unsafe code. A spinlock is a synchronization primitive that repeatedly checks for a condition to be met, making it suitable for scenarios where threads are expected to wait for a short duration.

## Features

- **Custom SpinLock Implementation**: The `SpinLock` struct provides a simple locking mechanism for shared data.
- **Thread-Safe**: The implementation ensures thread safety using `AtomicBool` and `UnsafeCell`.
- **RAII Guard**: The `Guard` struct ensures that the lock is released when it goes out of scope.

## Code Example

Here is an example of how to use the `SpinLock`:

```rust
let lock = SpinLock::new(Vec::new());
thread::scope(|s| {
    s.spawn(|| {
        lock.lock().push(1);
    });

    s.spawn(|| {
        let mut g = lock.lock();
        g.push(2);
        g.push(3);
    });
});

let g = lock.lock();
println!("{:?}", g.as_slice());
```

## Running the Example

To run the example, use the following command:

```bash
cargo run
```

## References

- [Rust Atomics and Locks](https://marabos.nl/atomics/)