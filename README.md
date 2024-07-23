# fakehub

[![DevOps By Rultor.com](http://www.rultor.com/b/h1alexbel/fakehub)](http://www.rultor.com/p/h1alexbel/fakehub)
[![We recommend IntelliJ IDEA](https://www.elegantobjects.org/intellij-idea.svg)](https://www.jetbrains.com/idea/)

[![just](https://github.com/h1alexbel/fakehub/actions/workflows/just.yml/badge.svg)](https://github.com/h1alexbel/fakehub/actions/workflows/just.yml)
[![Crates.io Version](https://img.shields.io/crates/v/fakehub)](https://crates.io/crates/fakehub)
[![codecov](https://codecov.io/gh/h1alexbel/fakehub/graph/badge.svg?token=0bcdqd2UKT)](https://codecov.io/gh/h1alexbel/fakehub)
[![PDD status](http://www.0pdd.com/svg?name=h1alexbel/fakehub)](http://www.0pdd.com/p?name=h1alexbel/fakehub)
[![Hits-of-Code](https://hitsofcode.com/github/h1alexbel/fakehub)](https://hitsofcode.com/view/github/h1alexbel/fakehub)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/h1alexbel/fakehub/blob/master/LICENSE.txt)
[![Known Vulnerabilities](https://snyk.io/test/github/h1alexbel/fakehub/badge.svg)](https://snyk.io/test/github/h1alexbel/fakehub)

fakehub - A fully functional fake version of a [GitHub API] that supports all
the features and works locally, with no connection to GitHub at all.

**Motivation**. There are many applications that use GitHub API for different
purposes. All of them need to create automated tests, which need to mock the
API server somehow. We offer a fully functioning mock version of a GitHub API,
which would support all functions, but work locally, with absolutely no
connection to GitHub.

## How to use?

First, install it from [crate][fakehub-crate]:

```bash
cargo install fakehub
```

or with [homebrew] (macOS):

```bash
brew install fakehub
```

Then, run it:

```bash
fakehub start --port 8080
```

Table of contents:

* [Overview](#overview)
* [Login](#login)
* [Repositories](#repositories)
* [Issues](#issues)
* [CLI Options](#cli-options)

### Overview

Fakehub is a full clone of [GitHub API]. This is very beneficial for testing,
when you should not use real GitHub API, but a mock version of it instead.
Fakehub stores all the data in [XML] format in file-system. When request
arrives, we query the storage, transform data into GitHub API-compatible format
([JSON]) and give it to you.

### Login

TBD..

### Repositories

TBD..

### Issues

TBD..

### CLI Options

You can use the following options within `fakehub` command-line tool:

| Name            | Value   | Default | Description                                                                                               |
|-----------------|---------|---------|-----------------------------------------------------------------------------------------------------------|
| `port`          | int     | `3000`  | Port to run fakehub server on.                                                                            |
| `v`             | boolean | `false` | Verbose run output, i.e. debug logs, etc.                                                                 |
| `report`        | boolean | `false` | Generate report after fakehub shutdown.                                                                   |
| `report-format` | string  | -       | Generated report format. Possible values: `latex` for [LaTeX], `xml` for [XML], and `txt` for plain text. |

## How to contribute?

Make sure that you have [Rust], [npm], Java 21+ and [just] installed on your
system, then fork this repository, make changes, send us a
[pull request][guidelines]. We will review your changes and apply them to the
`master` branch shortly, provided they don't violate our quality standards. To
avoid frustration, before sending us your pull request please run full build:

```bash
just full
```

Here is the [contribution vitals][Zerocracy Vitals], made by [zerocracy/judges-action]
(updated every hour!).

[GitHub API]: https://docs.github.com/en/rest?apiVersion=2022-11-28
[homebrew]: https://brew.sh
[fakehub-crate]: https://crates.io/crates/fakehub
[LaTeX]: https://en.wikipedia.org/wiki/LaTeX
[XML]: https://en.wikipedia.org/wiki/XML
[JSON]: https://en.wikipedia.org/wiki/JSON
[Rust]: https://www.rust-lang.org/tools/install
[npm]: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
[guidelines]: https://www.yegor256.com/2014/04/15/github-guidelines.html
[just]: https://just.systems/man/en/chapter_4.html
[Zerocracy Vitals]: https://www.h1alexbel.xyz/fakehub/zerocracy/fakehub-vitals.html
[zerocracy/judges-action]: https://github.com/zerocracy/judges-action
