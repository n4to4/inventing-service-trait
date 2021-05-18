use toy_service::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = Server::new("127.0.0.1:3000").await;
    server.run(handle_request).await?;
    Ok(())
}

async fn handle_request(request: HttpRequest) -> Result<HttpResponse, std::io::Error> {
    if request.path() == "/" {
        Ok(HttpResponse::ok("Hello, World!"))
    } else if request.path() == "/important-data" {
        let some_data = fetch_data_from_database().await;
        Ok(make_response(some_data))
    } else {
        Ok(HttpResponse::not_found())
    }
}

struct SomeData;

async fn fetch_data_from_database() -> SomeData {
    SomeData
}

fn make_response(_some_data: SomeData) -> HttpResponse {
    HttpResponse::ok("ok")
}
