# kq

[![Crates.io - version](https://img.shields.io/crates/v/kq)](https://crates.io/crates/kq)
[![Crates.io - download](https://img.shields.io/crates/d/kq)](https://crates.io/crates/kq)
[![docs.rs](https://img.shields.io/docsrs/kq)](https://docs.rs/kq)

A jq-like cli tool that can [query](https://github.com/kdl-org/kdl/blob/1.0.0/QUERY-SPEC.md) and transform [KDL](https://kdl.dev/) document right in the command line.

> `||` and [Map Operator](https://github.com/kdl-org/kdl/blob/1.0.0/QUERY-SPEC.md#map-operator) are not supported yet.

## Installation

### Pre-built binaries

You can find prebuilt binaries for Linux, macOS and Windows in the [release page](https://github.com/jihchi/kq/releases).

### via Cargo

```console
$ cargo install kq
```

## Usage

```console
$ kq -h
Usage: kq [options] <selector>

Options:
    -h, --help          print this help menu
    -v, --version       print the version
```

## Examples

> Modified from https://github.com/kdl-org/kdl/blob/1.0.0/QUERY-SPEC.md#examples

Given following content:

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
```

```console
$ cat example.kdl | kq "package name"
name "foo"
```

```console
$ cat example.kdl | kq "dependencies"
dependencies platform="windows" {
    winapi "1.0.0" path="./crates/my-winapi-fork"
}
dependencies {
    miette "2.0.0" dev=true
}
```

```console
$ cat example.kdl | kq "dependencies[platform]"
dependencies platform="windows" {
    winapi "1.0.0" path="./crates/my-winapi-fork"
}
```

```console
$ cat example.kdl | kq "dependencies > []"
winapi "1.0.0" path="./crates/my-winapi-fork"
miette "2.0.0" dev=true
```
