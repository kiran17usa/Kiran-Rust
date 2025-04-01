# A binary
cargo new foo

# A library
cargo new --lib bar

.
├── bar
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── foo
    ├── Cargo.toml
    └── src
        └── main.rs

//cargo.toml in foo 
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]


//add dependencies like below
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # from crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
bar = { path = "../bar" } # from a path in the local filesystem


//looks for prebuild compilationbefore things
[package]
...
build = "build.rs"