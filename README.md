# fakehub

[![EO principles respected here](https://www.elegantobjects.org/badge.svg)](https://www.elegantobjects.org)
[![DevOps By Rultor.com](http://www.rultor.com/b/h1alexbel/fakehub)](http://www.rultor.com/p/h1alexbel/fakehub)
[![We recommend IntelliJ IDEA](https://www.elegantobjects.org/intellij-idea.svg)](https://www.jetbrains.com/idea/)

[![just](https://github.com/h1alexbel/fakehub/actions/workflows/just.yml/badge.svg)](https://github.com/h1alexbel/fakehub/actions/workflows/just.yml)
[![Crates.io Version](https://img.shields.io/crates/v/fakehub)](https://crates.io/crates/fakehub)
[![codecov](https://codecov.io/gh/h1alexbel/fakehub/graph/badge.svg?token=0bcdqd2UKT)](https://codecov.io/gh/h1alexbel/fakehub)
[![PDD status](http://www.0pdd.com/svg?name=h1alexbel/fakehub)](http://www.0pdd.com/p?name=h1alexbel/fakehub)
[![Hits-of-Code](https://hitsofcode.com/github/h1alexbel/fakehub)](https://hitsofcode.com/view/github/h1alexbel/fakehub)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/h1alexbel/fakehub/blob/master/LICENSE.txt)
[![Known Vulnerabilities](https://snyk.io/test/github/h1alexbel/fakehub/badge.svg)](https://snyk.io/test/github/h1alexbel/fakehub)

fakehub - A fully functional fake version of a [GitHub REST API] that supports all
the features and works locally, with no connection to GitHub at all.

**Motivation**. There are many applications that use GitHub API for different
purposes. All of them need to create automated tests, which need to mock the
API server somehow. We offer a fully functioning mock version of a GitHub REST
API, which would support all functions, but work locally, with absolutely no
connection to GitHub.

## Quick start

First, install it from [crate][fakehub-crate]:

```bash
cargo install fakehub
```

Then, create a simple test, `test.sh`:

```bash
{
  fakehub start -d
  out="$(curl -s 'http://localhost:3000/users/jeff' | jq -r '.login')"
  expected="jeff"
  if [ "$out" == $expected ]; then
    echo "Test passed!"
  else
    echo "Login '$out' does not match with expected: '$expected'"
    exit 1
  fi
  fakehub stop
}
```

And run it:

```bash
sh test.sh
```

You should be able to see this:

```text
2024-10-15T15:14:33.469924Z  INFO fakehub: Starting server on port 3000
2024-10-15T15:14:33.470238Z  INFO fakehub: Server is running in detached mode on port 3000
2024-10-15T15:14:33.470247Z  INFO fakehub_server::sys::sys_info: OS: macos
2024-10-15T15:14:33.470251Z  INFO fakehub_server::sys::sys_info: PID: 11751
Test passed!
2024-10-15T15:14:33.486892Z  INFO fakehub: Stopping fakehub...
2024-10-15T15:14:33.518901Z  INFO fakehub::sys::kill_unix: Port 3000 killed
2024-10-15T15:14:33.518913Z  INFO fakehub: fakehub stopped
```

## Table of contents

* [Overview](#overview)
* [Request format](#request-format)
* [GitHub objects](#github-objects)
* [CLI options](#cli-options)

### Overview

fakehub is a full clone of [GitHub REST API]. This is very beneficial for
testing, when you should not use real GitHub, but a mock version of it instead.
fakehub stores all the data in memory. When [request arrives](#request-format),
we query the storage, transform objects into GitHub API-compatible format
([JSON]) and give it to you.

### Request format

fakehub supports the format specified in [GitHub REST API docs][GitHub REST API].
For instance, if you want to use [Get a repository][GitHub REST API Get Repo]
endpoint: you should just replace `api.github.com` to `localhost:$port` (make
sure that `fakehub` is running on specified port).

```bash
curl -L \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer <YOUR-TOKEN>" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  http://localhost:$port/repos/OWNER/REPO
```

**Attention!** Don't use your own GitHub Personal Access Tokens to authorize in
fakehub. Instead, use generated token by fakehub:

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"login": "jeff"}' \
  http://localhost:$port/login
```

This should generate you an access token to fakehub API.

### GitHub objects

TBD..

### CLI options

You can use the following options within `fakehub` command-line tool:

| Name              | Value   | Default | Description                                                                                               |
|-------------------|---------|---------|-----------------------------------------------------------------------------------------------------------|
| `-p`, `--port`    | int     | `3000`  | Port to run fakehub server on.                                                                            |
| `-v`, `--verbose` | boolean | `false` | Verbose run output, i.e. debug logs, etc.                                                                 |
| `-d`,  `--detach` | boolean | `false` | Run fakehub server in detached mode.                                                                      |
| `-i`, `--init`    | string  | -       | Path to file or directory with initial state.                                                             |
| `-r`, `--report`  | boolean | `false` | Generate report after fakehub shutdown.                                                                   |
| `--report-format` | string  | -       | Generated report format. Possible values: `latex` for [LaTeX], `xml` for [XML], and `txt` for plain text. |

## How to contribute?

Make sure that you have [Rust], [just], [npm], and Java 21+ installed on your
system, then fork this repository, make changes, send us a
[pull request][guidelines]. We will review your changes and apply them to the
`master` branch shortly, provided they don't violate our quality standards. To
avoid frustration, before sending us your pull request please run full build:

```bash
just full
```

Here is the [contribution vitals][Zerocracy Vitals], made by [zerocracy/judges-action]
(updated every hour!).

[GitHub REST API]: https://docs.github.com/en/rest?apiVersion=2022-11-28
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
[GitHub REST API Get Repo]: https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#get-a-repository
