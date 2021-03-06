// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn foo() {}
fn bar<T>() {}

fn main() {
    foo::<f64>(); //~ ERROR expected at most 0 type parameters, found 1 type parameter [E0087]
                  //~^ NOTE expected 0 type parameters

    bar::<f64, u64>(); //~ ERROR expected at most 1 type parameter, found 2 type parameters [E0087]
                       //~^ NOTE expected 1 type parameter
}
