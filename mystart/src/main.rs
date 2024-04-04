// Create a web server that listens on port 8080 and responds with "Hello, world!" to all requests.

// Add the actix-web dependency
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if let Err(error) = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello, world!") }))
    })
    .bind("127.0.0.1:8000") {
        eprintln!("Failed to bind to address: {}", error);
        return Err(error.into());
    }
    
    Ok(())
}
