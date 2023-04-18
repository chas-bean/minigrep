# Minigrep

[![Rust](https://github.com/chas-bean/minigrep/actions/workflows/test.yml/badge.svg)](https://github.com/chas-bean/minigrep/actions/workflows/test.yml)

Practice project from the Book of Rust, Chapter 12. The project is to make a mini version
of the `grep` command line utility

```
cargo run -- searchstring filename.txt
```

## Ignoring Case

You can ignore casing by specifying an environment variable

```
MINIGREP_IGNORE_CASE=1 cargo run -- searchstring filename.txt
```
