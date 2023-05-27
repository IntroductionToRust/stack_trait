# Stack Trait

This repo defines a simple stack trait. This is use in the context of the following
class: *Introduction to programming with Rust* (see https://tu-dresden.de/ing/informatik/sya/se/studium/lehrveranstaltungen/summer-semester/programming/index).

## Building

To check, build, and document, just execute:

```bash
./build.sh
```

This command will fail, in case not all required tools are installed. Please install the missing tools with the help of `cargo`.

## Documentation

After building the project, you can view the documentation in your browser by opening the following file `target/doc/stack_trait/index.html` or by executing:

```bash
cargo doc --open
```
