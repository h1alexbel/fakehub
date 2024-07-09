# fakehub

[![DevOps By Rultor.com](http://www.rultor.com/b/h1alexbel/fakehub)](http://www.rultor.com/p/h1alexbel/fakehub)
[![We recommend IntelliJ IDEA](https://www.elegantobjects.org/intellij-idea.svg)](https://www.jetbrains.com/idea/)

[![cargo](https://github.com/h1alexbel/fakehub/actions/workflows/cargo.yml/badge.svg)](https://github.com/h1alexbel/fakehub/actions/workflows/cargo.yml)
[![Crates.io Version](https://img.shields.io/crates/v/fakehub)](https://crates.io/crates/fakehub)
[![codecov](https://codecov.io/gh/h1alexbel/fakehub/graph/badge.svg?token=0bcdqd2UKT)](https://codecov.io/gh/h1alexbel/fakehub)
[![PDD status](http://www.0pdd.com/svg?name=h1alexbel/fakehub)](http://www.0pdd.com/p?name=h1alexbel/fakehub)
[![Hits-of-Code](https://hitsofcode.com/github/h1alexbel/fakehub)](https://hitsofcode.com/view/github/h1alexbel/fakehub)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/h1alexbel/fakehub/blob/master/LICENSE.txt)
[![Known Vulnerabilities](https://snyk.io/test/github/h1alexbel/fakehub/badge.svg)](https://snyk.io/test/github/h1alexbel/fakehub)

fakehub - A fully functional fake version of a GitHub API that supports all the
features and works locally, with no connection to GitHub at all.

**Motivation**. There are many applications that use GitHub API for different
purposes. All of them need to create automated tests, which need to mock the
API server somehow. We offer a fully functioning mock version of a GitHub API,
which would support all functions, but work locally, with absolutely no
connection to GitHub.

## How to use?

TBD..

## How to contribute?

Make sure that you have [Rust] and [just] installed on your system, then fork
this repository, make changes, send us a [pull request][guidelines].
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration, before
sending us your pull request please run full build:

```bash
just full
```

[Rust]: https://www.rust-lang.org/tools/install
[guidelines]: https://www.yegor256.com/2014/04/15/github-guidelines.html
[just]: https://just.systems/man/en/chapter_4.html
