use actix_web::{App, get, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index()->impl Responder {
    HttpResponse::Ok().body("myindex")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    }).bind(("0.0.0.0", 8080))?.run().await
}