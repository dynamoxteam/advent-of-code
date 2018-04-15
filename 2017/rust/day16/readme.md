# Day 16: Permutation Promenade

## Usage

In this particular day I used a
[feature](https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right)
from the nightly version of Rust, because it made so much easier to implement
the spin operation. Because of that, we need to have nightly Rust installed, and
add "+nightly" to the cargo commands in the shell in order to run the program.

```shell
cargo +nightly run <input file>
```

## Test

```shell
cargo +nightly test
```