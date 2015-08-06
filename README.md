An attempt to create safe Rust LLVM bindings ([rust-lang/rfcs#644](https://github.com/rust-lang/rfcs/issues/644)).

Primarily will be used in my [LLVM Kaleidoscope tutorial for Rust](https://github.com/jauhien/iron-kaleidoscope).

It is the very beginning of work on safe bindings. Pull requests are welcome.

TODO list (random points to do in the nearest time):

* memory manager for ExecutionEngine
* full ExecutionEngine API
* ORC
* change all methods to use static dispatch where appropriate (as in GenericValue)
