// Copyright (c) 2022 Vadim Glinka
//
// See the COPYRIGHT file at the top-level directory of this distribution
// and at https://github.com/vglinka/assert_eq_all/blob/main/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export] macro_rules!
assert_eq_all {
    // 0 args
    () => ();
    // 1 arg
    ( $arg:expr $(,)? ) => ();
    // 2 args
    ( $arg_1:expr, $arg_2:expr $(,)? ) => {
        assert_eq!($arg_1, $arg_2, "\n({} != {})", stringify!($arg_1), stringify!($arg_2));
    };
    // 3 args
    ( $arg_1:expr, $arg_2:expr, $arg_3:expr $(,)? ) => {        
        assert_eq!($arg_1, $arg_2, "\n({} != {})", stringify!($arg_1), stringify!($arg_2));
        assert_eq!($arg_2, $arg_3, "\n({} != {})", stringify!($arg_2), stringify!($arg_3));
    };
    // 4 args
    ( $arg_1:expr, $arg_2:expr, $arg_3:expr, $arg_4:expr $(,)? ) => {        
        assert_eq!($arg_1, $arg_2, "\n({} != {})", stringify!($arg_1), stringify!($arg_2));
        assert_eq!($arg_2, $arg_3, "\n({} != {})", stringify!($arg_2), stringify!($arg_3));
        assert_eq!($arg_3, $arg_4, "\n({} != {})", stringify!($arg_3), stringify!($arg_4));
    };
    // Any number of arguments.
    // At each level of recursion, we subtract 3 arguments from
    // the total number of arguments. At the end, we will have
    // 3 left, or 2, or the pattern will match 4 arguments.
    // But we will never get 1 argument. Because
    // 5 - 3 = 2
    // 6 - 3 = 3
    // 7 - 3 = 4
    // 8 - 3 = 5
    // ...
    ( $arg_1:expr, $arg_2:expr, $arg_3:expr, $($args:expr),* $(,)? ) => {        
        assert_eq!($arg_1, $arg_2, "\n({} != {})", stringify!($arg_1), stringify!($arg_2));
        assert_eq!($arg_2, $arg_3, "\n({} != {})", stringify!($arg_2), stringify!($arg_3));
        assert_eq_all!($($args),*,);
    };
}

#[macro_export] macro_rules!
assert_ne_all {
    // 0 args
    () => ();
    // 1 arg
    ( $arg:expr $(,)? ) => ();
    // 2 args
    ( $arg_1:expr, $arg_2:expr $(,)? ) => {
        assert_ne!($arg_1, $arg_2, "\n({} == {})", stringify!($arg_1), stringify!($arg_2));
    };
    // 3 args
    ( $arg_1:expr, $arg_2:expr, $arg_3:expr $(,)? ) => {        
        assert_ne!($arg_1, $arg_2, "\n({} == {})", stringify!($arg_1), stringify!($arg_2));
        assert_ne!($arg_2, $arg_3, "\n({} == {})", stringify!($arg_2), stringify!($arg_3));
    };
    // 4 args
    ( $arg_1:expr, $arg_2:expr, $arg_3:expr, $arg_4:expr $(,)? ) => {        
        assert_ne!($arg_1, $arg_2, "\n({} == {})", stringify!($arg_1), stringify!($arg_2));
        assert_ne!($arg_2, $arg_3, "\n({} == {})", stringify!($arg_2), stringify!($arg_3));
        assert_ne!($arg_3, $arg_4, "\n({} == {})", stringify!($arg_3), stringify!($arg_4));
    };
    // Any number of arguments.
    // At each level of recursion, we subtract 3 arguments from
    // the total number of arguments. At the end, we will have
    // 3 left, or 2, or the pattern will match 4 arguments.
    // But we will never get 1 argument. Because
    // 5 - 3 = 2
    // 6 - 3 = 3
    // 7 - 3 = 4
    // 8 - 3 = 5
    // ...
    ( $arg_1:expr, $arg_2:expr, $arg_3:expr, $($args:expr),* $(,)? ) => {        
        assert_ne!($arg_1, $arg_2, "\n({} == {})", stringify!($arg_1), stringify!($arg_2));
        assert_ne!($arg_2, $arg_3, "\n({} == {})", stringify!($arg_2), stringify!($arg_3));
        assert_ne_all!($($args),*,);
    };
}

