use hyper::service::{make_service_fn, Service};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

struct Config {
    message: String,
}

/// The service
struct TestService {
    config: Arc<Config>,
}

impl Service<Request<Body>> for TestService {
    type Response = Response<Body>;
    type Error = http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        let s = format!("Hello, {}!", self.config.message);
        let response = Response::new(Body::from(s));

        // create a response in a future.
        let fut = async { Ok(response) };

        // Return the response as an immediate future
        Box::pin(fut)
    }
}

#[tokio::main]
async fn main() {
    let config = Arc::new(Config {
        message: "world".to_owned(),
    });

    // Spawn the root task
    let make_svc = make_service_fn(move |_conn| {
        let clone = Arc::clone(&config);
        async {
            let service = TestService { config: clone };
            Ok::<_, Infallible>(service)
        }
    });

    // This is our socket address...
    let addr = ([127, 0, 0, 1], 4000).into();

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
