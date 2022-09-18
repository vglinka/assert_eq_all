## assert_eq_all macro

Accepts any number of arguments and panics if they are not equal.
The number of arguments can also be odd. Each argument will be compared
with the previous and following.

Additionally, the macro prints the names of the arguments that caused
the error, not just their values.

This is useful if you want to test whether different implementations
actually return the same result.

```sh
Hello world!
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `20`,
 right: `21`: 
(my_func_impl_6(x) != my_func_impl_7(x))', main.rs:18:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

**main.rs**

```rust
use assert_eq_all::{assert_eq_all, assert_ne_all};

// Benchmarked functions
fn my_func_impl_1(x: u32) -> u32 { x }
fn my_func_impl_2(x: u32) -> u32 { x }
fn my_func_impl_3(x: u32) -> u32 { x }
fn my_func_impl_4(x: u32) -> u32 { x }
fn my_func_impl_5(x: u32) -> u32 { x }
fn my_func_impl_6(x: u32) -> u32 { x }
fn my_func_impl_7(x: u32) -> u32 { x + 1 }
//                                   ^^^
// An error in one of the options for an alternative
// implementation of the algorithm.

fn main() {
    println!("Hello world!");              
    let x = 20;
    assert_eq_all!(
        my_func_impl_1(x), // 20
        my_func_impl_2(x), // 20
        my_func_impl_3(x), // 20
        my_func_impl_4(x), // 20
        my_func_impl_5(x), // 20
        my_func_impl_6(x), // 20
        my_func_impl_7(x), // 21 <--
    );

    assert_ne_all!(
        my_func_impl_6(x), // 20
        my_func_impl_7(x), // 21
    );
}
```
