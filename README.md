# Armadillo
#### A JavaScript interpreter written in Rust

Armadillo is a fast, easy-to-use interpreter for ECMAScript (JavaScript).
It is written in Rust.

## Goals
**Performance**: Armadillo should be as fast as leading JavaScript interpeters
like SpiderMonkey and V8. This may eventually require a transition to JIT
(just-in-time) compilation.

**Compliance**: Armadillo should be completely standards-compliant. Right now,
Armadillo is targeting the ECMAScript 2020 standard.

**Code cleanliness**: Armadillo should be thoroughly documented and tested. It
should be easy for experienced developers to understand what any segment of
code is doing.

**Ease of use**: Armadillo should be easy to use in other projects. It should
be used through its command line interface. At a later date, or perhaps in a
separate project, Armadillo should provide an environment that allows for
manipulation of webpages.

## Project Status
This project is currently in the earliest stages of development. No working
interpreter is available yet.

## Quick Start
```sh
$ git clone https://github.com/thomasebsmith/armadillo
$ cd armadillo
$ cargo run
```
