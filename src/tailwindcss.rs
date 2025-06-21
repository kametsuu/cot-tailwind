use std::process::Command;
use std::sync::Once;
use std::task::{Context, Poll};

use cot::Error;
use futures::future::BoxFuture;
use tower::Service;
use cot::response::Response;
use cot::request::Request;

/// Middleware that compiles TailwindCSS files using the Tailwind CLI.
#[derive(Debug, Clone)]
pub struct TailwindMiddleware;

impl TailwindMiddleware {
    pub fn new() -> Self {
        TailwindMiddleware
    }
}

impl<S> tower::Layer<S> for TailwindMiddleware {
    type Service = TailwindService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TailwindService::new(inner)
    }
}

#[derive(Debug, Clone)]
pub struct TailwindService<S> {
    inner: S,
}

impl<S> TailwindService<S> {
    fn new(inner: S) -> Self {
        static INIT: Once = Once::new();

        // Run the Tailwind CLI once when the server starts.
        INIT.call_once(|| {
            println!("Running TailwindCSS preprocessor...");

            let status = Command::new("tailwindcss")
                .args([
                    "-i", "static/css/tailwind.css",
                    "-o", "static/gen/main.css",
                ])
                .status();

            match status {
                Ok(status) if status.success() => println!("✅ TailwindCSS compiled successfully."),
                Ok(status) => eprintln!("❌ TailwindCSS failed: {}", status),
                Err(err) => eprintln!("❌ Failed to run tailwindcss.exe: {}", err),
            }
        });

        Self { inner }
    }
}

impl<S> Service<Request> for TailwindService<S>
where
    S: Service<Request, Response = Response, Error = Error> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = Error;
    type Future = BoxFuture<'static, Result<Response, Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            inner.call(req).await
        })
    }
}
