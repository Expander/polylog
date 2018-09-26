Polylog
=======

The Polylog package provides Rust implementations of real and complex
polylogarithms.

The Polylog package depends on the `num` crate.


Example
-------

```rust
extern crate num;
extern crate polylog;
use num::complex::Complex;
use polylog::{Li2, Li3};

fn main() {
    let x = 1.0;
    let z = Complex::new(1.0, 1.0);
    println!("Li2({}) = {}", x, x.li2());
    println!("Li2({}) = {}", z, z.li2());
    println!("Li3({}) = {}", z, z.li3());
}
```


Notes
-----

The implementation of the real dilogarithm has been taken from the
[ROOT](root.cern.ch) package, licensed under the GNU LGPL.

The implementation of the complex dilogarithm has been taken from
[SPheno](spheno.hepforge.org) and has been translated to Rust.


Copying
-------

Polylog is licenced under the GNU Lesser General Public License (GNU
LGPL) version 3.
