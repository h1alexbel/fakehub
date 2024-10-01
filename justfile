# The MIT License (MIT)
#
# Copyright (c) 2024 Aliaksei Bialiauski
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included
# in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

# Install required tools
install:
  cargo install cargo-machete
  cargo install killport
  npm install @openapitools/openapi-generator-cli@2.13.7 -g

# Full build.
full:
  just gen
  just build

# Generate code.
gen:
  cd github-mirror && \
    ${RULTOR:+sudo} openapi-generator-cli generate -i github.yml -g rust -o .

# Build the project.
build:
  cargo clean
  just test
  just check
  cargo build

# Run tests.
test:
  cargo test

# Check the quality of code.
check:
  cargo clippy --all-targets
  cargo +nightly fmt --check
  cargo machete

# Rultor merge script.
rultor:
  just gen
  cargo --color=never test
  cargo +nightly fmt --check -- --color=never
  cargo machete
  cargo doc --no-deps
  cargo clippy
