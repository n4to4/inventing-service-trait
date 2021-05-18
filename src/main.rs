use std::future::Future;
use std::io::{Error, ErrorKind};
use std::pin::Pin;
use std::time::Duration;
use toy_service::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let server = Server::new("127.0.0.1:3000").await;
    server.run(handle_request).await?;
    Ok(())
}

async fn handle_request(request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.path() == "/" {
        Ok(HttpResponse::ok("Hello, World!"))
    } else if request.path() == "/important-data" {
        let some_data = fetch_data_from_database().await;
        Ok(make_response(some_data))
    } else {
        Ok(HttpResponse::not_found())
    }
}

async fn handler_with_timeout(request: HttpRequest) -> Result<HttpResponse, Error> {
    let result = tokio::time::timeout(Duration::from_secs(30), handle_request(request)).await;

    match result {
        Ok(Ok(response)) => Ok(response),
        Ok(Err(error)) => Err(error),
        Err(_timeout_elapsed) => Err(Error::new(ErrorKind::Other, "timeout")),
    }
}

async fn handler_with_timeout_and_content_type(
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut response = handler_with_timeout(request).await?;
    response.set_header("Content-Type", "application/json");
    Ok(response)
}

struct SomeData;

async fn fetch_data_from_database() -> SomeData {
    SomeData
}

fn make_response(_some_data: SomeData) -> HttpResponse {
    HttpResponse::ok("ok")
}

//
// Handler impl
//

#[derive(Clone)]
struct RequestHandler;

impl Handler for RequestHandler {
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, Error>>>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        Box::pin(async move {
            if request.path() == "/" {
                Ok(HttpResponse::ok("Hello, World!"))
            } else if request.path() == "/important-data" {
                let some_data = fetch_data_from_database().await;
                Ok(make_response(some_data))
            } else {
                Ok(HttpResponse::not_found())
            }
        })
    }
}

#[derive(Clone)]
struct Timeout<T> {
    inner_handler: T,
    duration: Duration,
}

impl<T> Handler for Timeout<T>
where
    T: Handler + Clone + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, Error>>>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        let mut this = self.clone();

        Box::pin(async move {
            let result =
                tokio::time::timeout(this.duration, this.inner_handler.call(request)).await;

            match result {
                Ok(Ok(response)) => Ok(response),
                Ok(Err(error)) => Err(error),
                Err(_timeout_elapsed) => Err(Error::new(ErrorKind::Other, "timeout")),
            }
        })
    }
}

#[derive(Clone)]
struct JsonContentType<T> {
    inner_handler: T,
}

impl<T> Handler for JsonContentType<T>
where
    T: Handler + Clone + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, Error>>>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        let mut this = self.clone();

        Box::pin(async move {
            let mut response = this.inner_handler.call(request).await?;
            response.set_header("Content-Type", "application/json");
            Ok(response)
        })
    }
}
