Suppose that we wanted to have two binaries in the same project, though. What then?

It turns out that cargo supports this. The default binary name is main, as we saw before, but you can add additional binaries by placing them in a bin/ directory:

foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs


along with main.rs,  my_other_bin.rs also used for as a another binary in foo project.
We will expect to compile this file as well.

