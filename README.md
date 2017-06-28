# llvm-rs

[![Build Status](https://travis-ci.org/vaartis/llvm-rs.svg?branch=master)](https://travis-ci.org/vaartis/llvm-rs)

This is a work-in-progress rust-ideomatic bindings to LLVM, i am doing this because
the best bindings are those in the compiler and they are higtly imperative, not rust's style at all.
This bindings are (or will be) trying to provide more rust-y way of dealing with LLVM, some bits
are taken from the original C++ design, some are from OCaml bindings, this crate *might* also provide
some additional abstractions over C api. Please note, that tests *must* run in a single thread,
because some LLVM things seem to be not very thread safe and segfaults if ran in the non-main thread.

Feel free to contribute and submit pull requests.
