use std::io::Error;
use tokio::net;

pub struct HttpRequest;

impl HttpRequest {
    pub fn path(&self) -> &str {
        ""
    }
}

pub struct HttpResponse;

impl HttpResponse {
    pub fn ok(_body: impl AsRef<str>) -> Self {
        HttpResponse
    }

    pub fn not_found() -> Self {
        HttpResponse
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub async fn new(addr: impl Into<String>) -> Self {
        Server { addr: addr.into() }
    }

    pub async fn run<F>(self, handler: F) -> Result<(), Error>
    where
        F: Fn(HttpRequest) -> HttpResponse,
    {
        let listener = net::TcpListener::bind(self.addr).await?;

        loop {
            let (mut stream, _addr) = listener.accept().await?;
            let request = read_http_request(&mut stream).await?;

            // Call the handler provided by the user
            let _response = handler(request);

            write_http_response(&mut stream).await?;
        }
    }
}

async fn read_http_request(_stream: &mut net::TcpStream) -> Result<HttpRequest, Error> {
    todo!()
}

async fn write_http_response(_stream: &mut net::TcpStream) -> Result<(), Error> {
    todo!()
}
