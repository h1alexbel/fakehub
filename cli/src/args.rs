// The MIT License (MIT)
//
// Copyright (c) 2024 Aliaksei Bialiauski
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use clap::Parser;

// @todo #13:15min Handle port argument
//  We should process the port argument and
//  pass it to the server on `start` command.
//  Start command should be added also with clap
// @todo #41:15min Add --report argument together with content type flags.
//  Together with --report option let's add flags for XML, TeX, and Text
//  reports. So user can generate report in many formats like this: --report
//  --tex --html.
// @todo #41:45min Parse content type flag from stdin. Let's pick report generation
//  function based on provided flags, such as: --xml, --tex, --txt.
#[derive(Parser, Debug)]
pub(crate) struct Args {
    /// The port to run
    #[arg(short, long, default_value_t = 3000)]
    pub(crate) port: usize,
}
