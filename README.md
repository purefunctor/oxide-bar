# oxide-bar
A simple framework for configuring `lemonbar` programmatically through
Rust.

I wanted to be able to configure `lemonbar` in a language with
high-level, low-cost abstractions, which Rust has.

Each section in the bar can be represented by some data type that
implements the `Section` trait. Its `start` method allows some string
to be passed to the `Renderer` to be piped to `lemonbar`.
