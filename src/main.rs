use futures::future;
use futures::future::IntoFuture;
use hyper::rt::Future;
use hyper::service::{NewService, Service};
use hyper::{Body, Request, Response, Server, StatusCode};

/// The service
pub struct TestService<'s> {
    s: &'s str,
}

impl<'s> Service for TestService<'s> {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = http::Error;
    type Future = Box<dyn Future<Item = Response<Body>, Error = Self::Error> + Send + 's>;

    fn call(&mut self, _req: Request<Self::ReqBody>) -> Self::Future {
        Box::new(
            Response::builder()
                .status(StatusCode::OK)
                .body(format!("Hello, {}!", &self.s).into())
                .into_future(),
        )
    }
}

/// The server
pub struct TestServer<'a> {
    s: &'a str,
}

impl<'s, 'a: 's> NewService for &'s TestServer<'a> {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = http::Error;
    type InitError = http::Error;
    type Future = Box<dyn Future<Item = Self::Service, Error = Self::Error> + Send + 's>;
    type Service = TestService<'s>;
    fn new_service(&self) -> Self::Future {
        Box::new(future::ok(TestService { s: &self.s }))
    }
}

impl<'a> TestServer<'a> {
    pub const fn new(s: &'_ str) -> TestServer<'_> {
        TestServer { s }
    }

    pub fn start(&'static self) {
        // This is our socket address...
        let addr = ([127, 0, 0, 1], 4000).into();

        let server = Server::bind(&addr)
            .serve(self)
            .map_err(|e| eprintln!("server error: {}", e));

        // Run this server for... forever!
        hyper::rt::run(server);
    }
}

static SERVER: TestServer = TestServer::new("world");

fn main() {
    SERVER.start();
}
