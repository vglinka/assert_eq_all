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
