use futures::future;
use futures::future::IntoFuture;
use hyper::rt::Future;
use hyper::service::{NewService, Service};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::sync::Arc;

struct Config {
    message: String,
}

/// The service
struct TestService {
    config: Arc<Config>,
}

impl Service for TestService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = http::Error;
    type Future = Box<dyn Future<Item = Response<Body>, Error = Self::Error> + Send>;

    fn call(&mut self, _req: Request<Self::ReqBody>) -> Self::Future {
        Box::new(
            Response::builder()
                .status(StatusCode::OK)
                .body(format!("Hello, {}!", &self.config.message).into())
                .into_future(),
        )
    }
}

/// The server
struct TestServer {
    config: Arc<Config>,
}

impl NewService for TestServer {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = http::Error;
    type InitError = http::Error;
    type Future = Box<dyn Future<Item = Self::Service, Error = Self::Error> + Send>;
    type Service = TestService;
    fn new_service(&self) -> Self::Future {
        Box::new(future::ok(TestService {
            config: self.config.clone(),
        }))
    }
}

impl TestServer {
    fn new(config: Config) -> Self {
        TestServer {
            config: Arc::new(config),
        }
    }

    fn start(self) -> ! {
        // This is our socket address...
        let addr = ([127, 0, 0, 1], 4000).into();

        let server = Server::bind(&addr)
            .serve(self)
            .map_err(|e| eprintln!("server error: {}", e));

        // Run this server for... forever!
        hyper::rt::run(server);
        std::process::exit(0)
    }
}

fn main() {
    let config = Config {
        message: "world".to_owned(),
    };
    let server: TestServer = TestServer::new(config);
    server.start();
}
