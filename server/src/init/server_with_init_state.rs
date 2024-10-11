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
use crate::{DtServer, Server};
use anyhow::Result;
use fsl::transpiler::errors::err_ast::ErrAst;
use fsl::transpiler::fsl_transpiler::Fslt;
use futures::future::BoxFuture;
use log::{debug, info};
use std::path::Path;

/// Bootstrap server.
pub struct ServerWithInitState {
    /// Origin.
    pub origin: DtServer,
    /// Path to file or directory with state.
    pub path: String,
}

impl ServerWithInitState {
    /// New.
    pub fn new(origin: DtServer, path: String) -> ServerWithInitState {
        ServerWithInitState { origin, path }
    }
}

impl Server for ServerWithInitState {
    /// Start.
    // @todo #45:60min Transform transpiled state from FSL AST to Fakehub objects.
    //  Now we simply logging transpiled AST, let's create Fakehub objects from
    //  obtained AST.
    fn start(&self) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            let transpiled =
                ErrAst::default(Fslt::file(Path::new(&self.path))).decorate();
            debug!("Transpiled {}: {}", self.path, transpiled);
            info!("Initialized {} as state", self.path);
            self.origin.start().await
        })
    }
}
