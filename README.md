# trash-utils-rs

[![crates.io](https://img.shields.io/crates/v/trash-utils.svg)](https://crates.io/crates/trash-utils)

A cross platform Rust library for interacting with the trash.

**Warning**: trash-utils is currently beta level and no guarantees are made about its integrity of managing your files. Additionally, it currently only works on Linux, and it only interacts with the home trash and does not properly handle trashing files if the files are on a different filesystem/partition as your home folder.

## Usage

Add the following to Cargo.toml:

```toml
[dependencies]
trash-utils = "0.2.0"
```

## Apps using trash-utils-rs

- [trash-cli](https://github.com/cjbassi/trash-cli)

## Related projects

- sindresorhus
  - [empty-trash](https://github.com/sindresorhus/empty-trash)
  - [trash](https://github.com/sindresorhus/trash)
- [trash](https://github.com/ArturKovacs/trash)
