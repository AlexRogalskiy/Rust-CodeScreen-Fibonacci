# Rust-CodeScreen-Fibonacci
Example CodeScreen Rust assessment that requires the candidate to return the nth element in the Fibonacci sequence.

The sequence is assumed to be `0-indexed`, with `fibonacci(0)` returning 0 and `fibonacci(1)` returning 1.

You need to implement the `calculate()` function in the [fibonacci.rs](src/fibonacci.rs) file.

The tests that are run and are visible to the candidate are located in [fibonacci_test.rs](tests/fibonacci_test.rs).

The tests that are run and are not visible to the candidate are located in [fibonacci_hidden_test.rs](tests/fibonacci_hidden_test.rs).

## Requirements

The [fibonacci_test.rs](tests/fibonacci_test.rs) file should not be modified. If you would like to add your own tests, you
can add these in a separate file in the `tests` folder.

All tests must use the `assert Macros` from the `Rust standard library`.

The `Cargo.toml` file should only be modified in order to add any third-party dependencies required for your solution.

The `Cargo 2018` edition must be used.

## Tests
Run `cargo build` to build the project and then run `cargo tests` to run the tests. These should all pass if your solution has been implemented correctly.

Good luck!
