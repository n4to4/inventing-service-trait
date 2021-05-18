use toy_service::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = Server::new("127.0.0.1:3000").await;
    server.run(handle_request).await?;
    Ok(())
}

fn handle_request(request: HttpRequest) -> HttpResponse {
    if request.path() == "/" {
        HttpResponse::ok("Hello, World!")
    } else {
        HttpResponse::not_found()
    }
}
