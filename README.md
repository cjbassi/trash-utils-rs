# trash-utils-rs

[![crates.io](https://img.shields.io/crates/v/trash-utils.svg)](https://crates.io/crates/trash-utils)
[![docs.rs](https://docs.rs/trash-utils/badge.svg)](https://docs.rs/trash-utils)

A Rust library for interacting with the OS's trash-can.

Supported platforms:

- Linux/*BSD: [The FreeDesktop.org Trash specification](https://specifications.freedesktop.org/trash-spec/trashspec-1.0.html)

**Warning**: trash-utils is currently beta level and no guarantees are made about its integrity of managing your files. Additionally, it currently does not properly handle trashing files if the files are on a different filesystem/partition than the home folder.

## Usage

Add the following to Cargo.toml:

```toml
[dependencies]
trash-utils = "0.2.0"
```

## Apps using trash-utils-rs

- [trash-man](https://github.com/cjbassi/trash-man)

## Related projects

- sindresorhus
  - [empty-trash](https://github.com/sindresorhus/empty-trash)
  - [trash](https://github.com/sindresorhus/trash)
- [trash](https://github.com/ArturKovacs/trash)
