# How to Run Assembly

----

## Running with Cargo

```bash
cargo run --bin <name> -- [args]
```
or set ```default-run``` within ```[package]``` of cargo.toml to just use ```cargo run```

## Installing to PATH with Cargo

```bash
cargo install --path . --bin <name>
```
or ```cargo install --path .``` installs all binaries by default unless ```--bin``` is defined

## Reinstalling with --force

```bash
cargo install --path . --force
```
Overwrites an existing install. Needed if the binary is already installed and you want to update it after making changes.

## Custom Install Location with --root

```bash
cargo install --path . --root /some/custom/dir
```
Installs to `/some/custom/dir/bin` instead of the default `~/.cargo/bin`. You'll need to add that directory to your PATH yourself if it isn't already there.
