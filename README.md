# kq

[![Crates.io - version](https://img.shields.io/crates/v/kq)](https://crates.io/crates/kq)
[![Crates.io - download](https://img.shields.io/crates/d/kq)](https://crates.io/crates/kq)
[![docs.rs](https://img.shields.io/docsrs/kq)](https://docs.rs/kq)

>  🚧 work in progress

A jq-like cli tool that can [query](https://github.com/kdl-org/kdl/blob/main/QUERY-SPEC.md) and transform [KDL](https://kdl.dev/) document right in the command line.

```console
$ cat example.kdl
package {
    name "foo"
    version "1.0.0"
    dependencies platform="windows" {
        winapi "1.0.0" path="./crates/my-winapi-fork"
    }
    dependencies {
        miette "2.0.0" dev=true
    }
}

$ cat example.kdl | kq "package name"
name "foo"

$ cat example.kdl | kq "dependencies"
dependencies platform="windows" {
    winapi "1.0.0" path="./crates/my-winapi-fork"
}
dependencies {
    miette "2.0.0" dev=true
}

$ cat example.kdl | kq "dependencies[platform]"
dependencies platform="windows" {
    winapi "1.0.0" path="./crates/my-winapi-fork"
}

$ cat example.kdl | kq "dependencies > []"
winapi "1.0.0" path="./crates/my-winapi-fork"
miette "2.0.0" dev=true

```

## Installation

### Cargo

```sh
cargo install kq
```

