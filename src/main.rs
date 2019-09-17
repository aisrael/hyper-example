use futures::future;
use hyper::rt::Future;

use hyper::service::{NewService, Service};
use hyper::{Body, Request, Response, Server, StatusCode};

/// The service
pub struct TestService {}

impl Service for TestService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

    fn call(&mut self, _req: Request<Self::ReqBody>) -> Self::Future {
        Box::new(future::ok(
            Response::builder()
                .status(StatusCode::OK)
                .body("Ok".into())
                .unwrap(),
        ))
    }
}

/// The server
pub struct TestServer {}

impl NewService for TestServer {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type InitError = hyper::Error;
    type Future = Box<Future<Item = Self::Service, Error = Self::InitError> + Send>;
    type Service = TestService;
    fn new_service(&self) -> Self::Future {
        Box::new(future::ok(TestService {}))
    }
}

impl TestServer {
    pub fn new() -> TestServer {
        TestServer {}
    }

    pub fn start(self) {
        // This is our socket address...
        let addr = ([127, 0, 0, 1], 4000).into();

        let server = Server::bind(&addr)
            .serve(self)
            .map_err(|e| eprintln!("server error: {}", e));

        // Run this server for... forever!
        hyper::rt::run(server);
    }
}

fn main() {
    let test = TestServer::new();
    test.start();
}
