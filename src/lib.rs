//! Exports metrics over HTTP.
//!
//! This exporter can utilize observers that are able to be converted to a textual representation
//! via [`Drain<String>`].  It will respond to any requests, regardless of the method or path.
//!
//! Awaiting on `async_run` will drive an HTTP server listening on the configured address.
#![deny(missing_docs)]

use async_std::net::TcpListener;
use async_std::prelude::*;
use metrics_core::{Builder, Drain, Observe, Observer};
use std::net::SocketAddr;

/// Exports metrics over HTTP.
pub struct HttpExporter<C, B> {
    controller: C,
    builder: B,
    address: SocketAddr,
}

impl<C, B> HttpExporter<C, B>
where
    C: Observe + Send + Sync + 'static,
    B: Builder + Send + Sync + 'static,
    B::Output: Drain<String> + Observer,
{
    /// Creates a new [`HttpExporter`] that listens on the given `address`.
    ///
    /// Observers expose their output by being converted into strings.
    pub fn new(controller: C, builder: B, address: SocketAddr) -> Self {
        HttpExporter {
            controller,
            builder,
            address,
        }
    }

    /// Starts an HTTP server on the `address` the exporter was originally configured with,
    /// responding to any request with the output of the configured observer.
    pub async fn async_run(self) -> std::io::Result<()> {
        let builder = self.builder;
        let controller = self.controller;

        let listener = TcpListener::bind(self.address).await?;
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let mut stream = stream?;

            // seems like we have to read before writing if not we get an "Connection reset by peer"
            // cUrl and firefox are ok with 512 bytes but Chrome needs some more just in case we read 1024 bytes.
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).await?;

            let mut observer = builder.build();
            controller.observe(&mut observer);
            let output = observer.drain();

            let response = format!("HTTP/1.1 200\r\n\r\n{}", output);
            stream.write_all(response.as_bytes()).await?;
        }

        Ok(())
    }
}